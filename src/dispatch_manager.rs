use vulkano::{device::{Device, Queue}, image::view::ImageView};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{self, AcquireError, Swapchain, SwapchainCreationError};
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, DynamicState},
    swapchain::Surface,
};

use winit::window::Window;

use super::presenter::Presenter;

use std::sync::Arc;

/// This method is called once during initialization, then again whenever the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(ImageView::new(image.clone()).unwrap())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

pub struct DispatchManager {
    device: Arc<Device>,
    queue: Arc<Queue>,
    presenter: Arc<Presenter>,
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,

    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    dynamic_state: DynamicState,
    recreate_swapchain: bool,
}

impl DispatchManager {
    pub fn new(
        device: Arc<Device>,
        queue: Arc<Queue>,
        presenter: Arc<Presenter>,
        surface: Arc<Surface<Window>>,
        swapchain: Arc<Swapchain<Window>>,
        swapchain_images: &Vec<Arc<SwapchainImage<Window>>>,
    ) -> DispatchManager {
        // Dynamic viewports allow us to recreate just the viewport when the window is resized
        // Otherwise we would have to recreate the whole pipeline.
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            ..Default::default()
        };

        let framebuffers = window_size_dependent_setup(
            swapchain_images,
            presenter.get_render_pass(),
            &mut dynamic_state,
        );

        DispatchManager {
            device,
            queue,
            presenter,
            surface,
            swapchain,

            framebuffers,
            dynamic_state,
            recreate_swapchain: false,
        }
    }

    pub fn create_and_submit_commands<F>(&mut self, creation_func: F) -> bool
    where
        F: FnOnce(AutoCommandBufferBuilder) -> AutoCommandBufferBuilder,
    {
        let window = self.surface.window();
        if self.recreate_swapchain {
            let size = window.inner_size();
            let dimensions = [size.width, size.height];

            let (new_swapchain, new_images) =
                match self.swapchain.recreate_with_dimensions(dimensions) {
                    Ok(r) => r,
                    // This error tends to happen when the user is manually resizing the window.
                    // Simply restarting the loop is the easiest way to fix this issue.
                    Err(SwapchainCreationError::UnsupportedDimensions) => return false,
                    Err(err) => panic!("{:?}", err),
                };

            self.swapchain = new_swapchain;
            self.framebuffers = window_size_dependent_setup(
                &new_images,
                self.presenter.get_render_pass(),
                &mut self.dynamic_state,
            );

            self.recreate_swapchain = false;
        }

        let (image_num, _, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return false;
                }
                Err(err) => panic!("{:?}", err),
            };

        let builder = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )
        .unwrap();
        let builder = creation_func(builder);
        let builder = self.presenter.add_present_commands(
            builder,
            &self.dynamic_state,
            self.framebuffers[image_num].clone(),
        );
        let command_buffer = builder.build().unwrap();
        let future = acquire_future
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        // If we did any intense CPU side comp, we would want to wait for the GPU to finish *after*
        // we do all the CPU comp. However, our computation is entirely GPU based so we can get
        // away with this tidier but bad practice solution.
        match future {
            Ok(future) => {
                future.wait(None).unwrap();
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        true
    }

    pub fn invalidate_swapchain(&mut self) {
        self.recreate_swapchain = true;
    }
}
