use vulkano::descriptor::descriptor_set::{DescriptorSet, PersistentDescriptorSet};
use vulkano::descriptor::pipeline_layout::PipelineLayout;
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::image::{ImageDimensions, StorageImage};
use vulkano::pipeline::ComputePipeline;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    descriptor::PipelineLayoutAbstract,
};
use vulkano::{command_buffer::AutoCommandBufferBuilder, image::view::ImageView};

use std::sync::Arc;

use crate::options::{PARAMETER_SPACE, WORLD_SIZE};
use crate::{dispatch_manager::DispatchManager, options::Options, shaders};

type RandomizePipeline = ComputePipeline<PipelineLayout<shaders::randomize::MainLayout>>;
type SimulatePipeline = ComputePipeline<PipelineLayout<shaders::simulate::MainLayout>>;
type FinalizePipeline = ComputePipeline<PipelineLayout<shaders::finalize::MainLayout>>;

type GenericImage = StorageImage<Format>;
type GenericDescriptorSet = dyn DescriptorSet + Sync + Send;

pub struct Renderer {
    target_width: u32,
    target_height: u32,

    world_buffer_source: Arc<GenericImage>,
    world_buffer_target: Arc<GenericImage>,
    cpu_world_buffer: Arc<CpuAccessibleBuffer<[u16]>>,

    parameter_buffer: Arc<CpuAccessibleBuffer<[i16]>>,
    parameter_image: Arc<GenericImage>,

    randomize_pipeline: Arc<RandomizePipeline>,
    randomize_descriptors: Arc<GenericDescriptorSet>,

    simulate_pipeline: Arc<SimulatePipeline>,
    simulate_descriptors: Arc<GenericDescriptorSet>,

    finalize_pipeline: Arc<FinalizePipeline>,
    finalize_descriptors: Arc<GenericDescriptorSet>,
}

struct RenderBuilder {
    device: Arc<Device>,
    queue: Arc<Queue>,
    target_image: Arc<GenericImage>,
}

impl RenderBuilder {
    fn build(self) -> Renderer {
        let (target_width, target_height) = match self.target_image.dimensions() {
            ImageDimensions::Dim2d { width, height, .. } => (width, height),
            _ => panic!("A non-2d image was passed as the target of a Renderer."),
        };

        let world_buffer_source = StorageImage::new(
            self.device.clone(),
            ImageDimensions::Dim2d {
                width: WORLD_SIZE,
                height: WORLD_SIZE,
                array_layers: 1,
            },
            Format::R16Uint,
            Some(self.queue.family()),
        )
        .unwrap();

        let world_buffer_target = StorageImage::new(
            self.device.clone(),
            ImageDimensions::Dim2d {
                width: WORLD_SIZE,
                height: WORLD_SIZE,
                array_layers: 1,
            },
            Format::R16Uint,
            Some(self.queue.family()),
        )
        .unwrap();

        let cpu_world_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            false,
            (0..WORLD_SIZE * WORLD_SIZE).map(|_| 0u16),
        )
        .unwrap();

        let parameter_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            false,
            (0..PARAMETER_SPACE).map(|_| 0i16),
        )
        .unwrap();
        let parameter_image = StorageImage::new(
            self.device.clone(),
            ImageDimensions::Dim1d {
                width: PARAMETER_SPACE as _,
                array_layers: 1,
            },
            Format::R16Uint,
            Some(self.queue.family()),
        )
        .unwrap();

        let randomize_shader = shaders::load_randomize_shader(self.device.clone());
        let simulate_shader = shaders::load_simulate_shader(self.device.clone());
        let finalize_shader = shaders::load_finalize_shader(self.device.clone());

        let randomize_pipeline = Arc::new(
            ComputePipeline::new(
                self.device.clone(),
                &randomize_shader.main_entry_point(),
                &(),
                None,
            )
            .unwrap(),
        );
        let randomize_descriptors: Arc<GenericDescriptorSet> = Arc::new(
            PersistentDescriptorSet::start(
                randomize_pipeline.descriptor_set_layout(0).unwrap().clone(),
            )
            .add_image(ImageView::new(world_buffer_source.clone()).unwrap())
            .unwrap()
            .build()
            .unwrap(),
        );

        let simulate_pipeline = Arc::new(
            ComputePipeline::new(
                self.device.clone(),
                &simulate_shader.main_entry_point(),
                &(),
                None,
            )
            .unwrap(),
        );
        let simulate_descriptors: Arc<GenericDescriptorSet> = Arc::new(
            PersistentDescriptorSet::start(
                simulate_pipeline.descriptor_set_layout(0).unwrap().clone(),
            )
            .add_image(ImageView::new(world_buffer_source.clone()).unwrap())
            .unwrap()
            .add_image(ImageView::new(world_buffer_target.clone()).unwrap())
            .unwrap()
            .add_image(ImageView::new(parameter_image.clone()).unwrap())
            .unwrap()
            .build()
            .unwrap(),
        );

        let finalize_pipeline = Arc::new(
            ComputePipeline::new(
                self.device.clone(),
                &finalize_shader.main_entry_point(),
                &(),
                None,
            )
            .unwrap(),
        );
        let finalize_descriptors: Arc<GenericDescriptorSet> = Arc::new(
            PersistentDescriptorSet::start(
                finalize_pipeline.descriptor_set_layout(0).unwrap().clone(),
            )
            .add_image(ImageView::new(world_buffer_source.clone()).unwrap())
            .unwrap()
            .add_image(ImageView::new(self.target_image.clone()).unwrap())
            .unwrap()
            .build()
            .unwrap(),
        );

        Renderer {
            target_width,
            target_height,

            randomize_pipeline,
            randomize_descriptors,

            parameter_buffer,
            parameter_image,

            world_buffer_source,
            world_buffer_target,
            cpu_world_buffer,

            simulate_pipeline,
            simulate_descriptors,

            finalize_pipeline,
            finalize_descriptors,
        }
    }
}

impl Renderer {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        target_image: Arc<GenericImage>,
    ) -> Renderer {
        RenderBuilder {
            device,
            queue,
            target_image,
        }
        .build()
    }

    pub fn render(&mut self, dispatcher: &mut DispatchManager, options: &Options) -> bool {
        if options.display {
            dispatcher
                .create_and_submit_commands(|builder| self.add_render_commands(builder, options))
        } else {
            dispatcher.do_commands_without_presenting(|builder| {
                self.add_render_commands(builder, options)
            })
        }
    }

    fn add_render_commands(
        &mut self,
        mut add_to: AutoCommandBufferBuilder,
        options: &Options,
    ) -> AutoCommandBufferBuilder {
        {
            let mut pbuf = self.parameter_buffer.write().unwrap();
            pbuf.copy_from_slice(&options.kernel_arguments[..]);
        }
        add_to
            .copy_buffer_to_image(self.parameter_buffer.clone(), self.parameter_image.clone())
            .unwrap();
        if options.reset {
            add_to
                .dispatch(
                    [WORLD_SIZE / 8, WORLD_SIZE / 8, 1],
                    self.randomize_pipeline.clone(),
                    self.randomize_descriptors.clone(),
                    (),
                    vec![],
                )
                .unwrap();
        }
        for _ in 0..options.rate + options.skip {
            add_to
                .dispatch(
                    [WORLD_SIZE / 8, WORLD_SIZE / 8, 1],
                    self.simulate_pipeline.clone(),
                    self.simulate_descriptors.clone(),
                    (),
                    vec![],
                )
                .unwrap()
                .copy_image(
                    self.world_buffer_target.clone(),
                    [0, 0, 0],
                    0,
                    0,
                    self.world_buffer_source.clone(),
                    [0, 0, 0],
                    0,
                    0,
                    [WORLD_SIZE, WORLD_SIZE, 1],
                    1,
                )
                .unwrap();
        }
        let push_data = shaders::finalize::ty::PushData {
            offset: options.offset,
            zoom: options.zoom,
        };
        if options.display {
            add_to
                .dispatch(
                    [self.target_width / 8, self.target_height / 8, 1],
                    self.finalize_pipeline.clone(),
                    self.finalize_descriptors.clone(),
                    push_data,
                    vec![],
                )
                .unwrap();
        }
        add_to
            .copy_image_to_buffer(
                self.world_buffer_target.clone(),
                self.cpu_world_buffer.clone(),
            )
            .unwrap();
        add_to
    }

    pub fn with_cpu_world_buffer<R>(&self, visitor: impl FnOnce(&[u16]) -> R) -> R {
        let slice = self.cpu_world_buffer.read().unwrap();
        visitor(&slice[..])
    }
}
