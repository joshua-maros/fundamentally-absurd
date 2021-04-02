use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::{PresentMode, Surface, SurfaceTransform, Swapchain};
use vulkano::{
    device::{Device, DeviceExtensions, Queue},
    swapchain::ColorSpace,
};

use vulkano_win::VkSurfaceBuild;

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use std::sync::Arc;

pub struct InitResult {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface: Arc<Surface<Window>>,
    pub events_loop: EventLoop<()>,
    pub swapchain: Arc<Swapchain<Window>>,
    pub swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
}

pub fn init() -> InitResult {
    let instance = {
        // We don't need anything fancy.
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).unwrap()
    };

    // Get the first physical device we find.
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    println!(
        "Using device: {} (type: {:?})",
        physical.name(),
        physical.ty()
    );

    // Setup the window.
    let events_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1024, 1024))
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    let window = surface.window();

    // Get queue families for the graphical portion and the compute portion.
    let queue_family = physical
        .queue_families()
        .find(|&q| {
            // We take the first queue that supports drawing to our window.
            q.supports_graphics()
                && q.supports_compute()
                && surface.is_supported(q).unwrap_or(false)
        })
        .unwrap();

    // Create the virtual device using the queues and basic extensions.
    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (device, mut queues) = Device::new(
        physical,
        physical.supported_features(),
        &device_ext,
        [(queue_family, 0.5)].iter().cloned(),
    )
    .unwrap();

    // Unwrap the created queues.
    let queue = queues.next().unwrap();

    // Make a swapchain.
    let (swapchain, swapchain_images) = {
        let caps = surface.capabilities(physical).unwrap();
        let usage = ImageUsage::color_attachment();

        // The alpha mode indicates how the alpha value of the final image will behave. For example
        // you can choose whether the window will be opaque or transparent.
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        // Choosing the internal format that the images will have.
        let format = caps.supported_formats[0].0;
        let dimensions = window.inner_size();
        let dimensions = [dimensions.width, dimensions.height];
        let initial_dimensions = dimensions;

        Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            initial_dimensions,
            1,
            usage,
            &queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            vulkano::swapchain::FullscreenExclusive::Disallowed,
            true,
            ColorSpace::SrgbNonLinear,
        )
        .unwrap()
    };

    InitResult {
        device,
        queue,
        surface,
        events_loop,
        swapchain,
        swapchain_images,
    }
}
