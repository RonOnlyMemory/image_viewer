use glutin::{surface::{Surface, WindowSurface}, context::{PossiblyCurrentContext, ContextAttributesBuilder, ContextApi, Version}, config::ConfigTemplateBuilder, display::GetGlDisplay, prelude::{GlDisplay, NotCurrentGlContextSurfaceAccessor}};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{event_loop::{EventLoop, EventLoopBuilder}, window::{Window, WindowBuilder}};

pub struct SimpleWindow {
	pub event_loop: EventLoop<()>,
	pub window: Window,
	pub surface: Surface<WindowSurface>,
	pub ctx: PossiblyCurrentContext,
	pub gl: glow::Context,
}

impl SimpleWindow {
	pub fn new() -> Option<Self> {
		let event_loop = EventLoopBuilder::new().build();
		let window_builder = WindowBuilder::new()
			.with_transparent(true)
			.with_visible(false)
			.with_maximized(false)
			.with_decorations(false);
		let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder.clone()));
		let template = ConfigTemplateBuilder::new()
			.with_transparency(true);
		let (window, gl_config) = display_builder
			.build(&event_loop, template, |configs| {
				configs
					.reduce(|accum, _config| accum)
					.unwrap()
			})
			.unwrap();
		let window = window.unwrap();
		let display = gl_config.display();
		let raw_window_handle = window.raw_window_handle();
		let context_attributes = ContextAttributesBuilder::new()
			.with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
			.build(Some(raw_window_handle));
		let ctx = unsafe {
			display.create_context(&gl_config, &context_attributes)
		}.unwrap();
		let attrs = window.build_surface_attributes(<_>::default());
		let surface = unsafe {
			gl_config.display().create_window_surface(&gl_config, &attrs).unwrap()
		};
		let ctx = ctx.make_current(&surface).unwrap();
		let gl = unsafe {
			glow::Context::from_loader_function_cstr(|addr| display.get_proc_address(addr))
		};
		Some(Self {
			event_loop,
			window,
			ctx,
			gl,
			surface,
		})
	}
}
