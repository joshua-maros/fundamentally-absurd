use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState, SubpassContents};
use vulkano::descriptor::descriptor_set::{DescriptorSet, PersistentDescriptorSet};
use vulkano::descriptor::pipeline_layout::PipelineLayoutAbstract;
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::{ImageDimensions, StorageImage};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    image::view::ImageView,
};

use std::sync::Arc;

use crate::shaders;

#[derive(Clone, Debug, Default)]
struct PresenterVertex {
    position: [f32; 2],
}

vulkano::impl_vertex!(PresenterVertex, position);

type PresenterVertexBuffer = CpuAccessibleBuffer<[PresenterVertex]>;
type PresenterIndexBuffer = CpuAccessibleBuffer<[u32]>;
type GenericImage = StorageImage<Format>;

type GenericRenderPass = dyn RenderPassAbstract + Sync + Send;
type PresenterGraphicsPipeline = GraphicsPipeline<
    SingleBufferDefinition<PresenterVertex>,
    Box<dyn PipelineLayoutAbstract + Send + Sync>,
    Arc<GenericRenderPass>,
>;
type GenericDescriptorSet = dyn DescriptorSet + Sync + Send;

pub struct Presenter {
    vertex_buffer: Arc<PresenterVertexBuffer>,
    index_buffer: Arc<PresenterIndexBuffer>,
    presented_image: Arc<GenericImage>,

    render_pass: Arc<GenericRenderPass>,
    graphics_pipeline: Arc<PresenterGraphicsPipeline>,
    graphics_descriptors: Arc<GenericDescriptorSet>,
}

struct PresenterBuilder {
    device: Arc<Device>,
    queue: Arc<Queue>,
    resolution: (u32, u32),
    format: Format,
}

impl PresenterBuilder {
    fn make_quad(&self) -> (Arc<PresenterVertexBuffer>, Arc<PresenterIndexBuffer>) {
        // Create a vertex buffer containing a full screen quad.
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            false,
            [
                PresenterVertex {
                    position: [1.0, 1.0],
                },
                PresenterVertex {
                    position: [-1.0, 1.0],
                },
                PresenterVertex {
                    position: [-1.0, -1.0],
                },
                PresenterVertex {
                    position: [1.0, -1.0],
                },
            ]
            .iter()
            .cloned(),
        )
        .unwrap();

        // Indexes buffer used when drawing the quad.
        let index_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::index_buffer(),
            false,
            [0u32, 1u32, 2u32, 2u32, 3u32, 0u32].iter().cloned(),
        )
        .unwrap();

        (vertex_buffer, index_buffer)
    }

    fn make_presented_image(&self) -> (Arc<GenericImage>, Arc<Sampler>) {
        // The image that the compute shader will write from and the graphics pipeline will read from.
        let presented_image = StorageImage::new(
            self.device.clone(),
            ImageDimensions::Dim2d {
                width: self.resolution.0,
                height: self.resolution.1,
                array_layers: 1,
            },
            Format::R8G8B8A8Unorm,
            Some(self.queue.family()),
        )
        .unwrap();

        let sampler = Sampler::new(
            self.device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::ClampToEdge,
            SamplerAddressMode::ClampToEdge,
            SamplerAddressMode::ClampToEdge,
            0.0,
            1.0,
            0.0,
            0.0,
        )
        .unwrap();

        (presented_image, sampler)
    }

    fn load_shaders(&self) -> (shaders::screen_vs::Shader, shaders::screen_fs::Shader) {
        (
            shaders::load_screen_vertex_shader(self.device.clone()),
            shaders::load_screen_fragment_shader(self.device.clone()),
        )
    }

    fn build(self) -> Presenter {
        let (vertex_buffer, index_buffer) = self.make_quad();
        let (presented_image, image_sampler) = self.make_presented_image();
        let (vertex_shader, fragment_shader) = self.load_shaders();

        let render_pass: Arc<GenericRenderPass> = Arc::new(
            vulkano::single_pass_renderpass!(
                self.device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: self.format,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );

        let graphics_pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<PresenterVertex>()
                .vertex_shader(vertex_shader.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fragment_shader.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(self.device.clone())
                .unwrap(),
        );

        let graphics_descriptors: Arc<GenericDescriptorSet> = Arc::new(
            PersistentDescriptorSet::start(
                graphics_pipeline.descriptor_set_layout(0).unwrap().clone(),
            )
            .add_sampled_image(
                ImageView::new(presented_image.clone()).unwrap(),
                image_sampler.clone(),
            )
            .unwrap()
            .build()
            .unwrap(),
        );

        Presenter {
            vertex_buffer,
            index_buffer,
            presented_image,

            render_pass,
            graphics_pipeline,
            graphics_descriptors,
        }
    }
}

impl Presenter {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        resolution: (u32, u32),
        format: Format,
    ) -> Presenter {
        PresenterBuilder {
            device,
            queue,
            resolution,
            format,
        }
        .build()
    }

    pub fn get_presented_image(&self) -> Arc<GenericImage> {
        self.presented_image.clone()
    }

    pub fn get_render_pass(&self) -> Arc<GenericRenderPass> {
        self.render_pass.clone()
    }

    pub fn add_present_commands(
        &self,
        mut add_to: AutoCommandBufferBuilder,
        state: &DynamicState,
        output: Arc<dyn FramebufferAbstract + Send + Sync>,
    ) -> AutoCommandBufferBuilder {
        let clear_values = vec![[1.0, 0.0, 1.0, 1.0].into()];
        add_to
            .begin_render_pass(
                output,
                SubpassContents::Inline,
                clear_values,
            )
            .unwrap()
            .draw_indexed(
                self.graphics_pipeline.clone(),
                state,
                self.vertex_buffer.clone(),
                self.index_buffer.clone(),
                self.graphics_descriptors.clone(),
                (),
                vec![],
            )
            .unwrap()
            .end_render_pass()
            .unwrap();
        add_to
    }
}
