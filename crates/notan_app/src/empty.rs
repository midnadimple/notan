use crate::config::WindowConfig;
use crate::{
    App, Backend, BackendSystem, CursorIcon, EventIterator, FrameState, InitializeFn, WindowBackend,
};
use notan_audio::AudioBackend;
use notan_graphics::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct EmptyWindowBackend {
    size: (i32, i32),
    is_fullscreen: bool,
    lazy: bool,
}

impl WindowBackend for EmptyWindowBackend {
    fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
    }

    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        self.is_fullscreen = enabled;
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    fn dpi(&self) -> f64 {
        1.0
    }

    fn set_lazy_loop(&mut self, lazy: bool) {
        self.lazy = lazy;
    }

    fn lazy_loop(&self) -> bool {
        self.lazy
    }

    fn request_frame(&mut self) {
        // no-op
    }

    fn set_cursor(&mut self, _cursor: CursorIcon) {}

    fn cursor(&self) -> CursorIcon {
        CursorIcon::Default
    }
}

#[derive(Default)]
pub struct EmptyBackend {
    window: EmptyWindowBackend,
}

impl EmptyBackend {
    pub fn new() -> Result<Self, String> {
        Ok(Default::default())
    }
}

impl Backend for EmptyBackend {
    fn window(&mut self) -> &mut dyn WindowBackend {
        &mut self.window
    }

    fn events_iter(&mut self) -> EventIterator {
        Default::default()
    }

    fn exit(&mut self) {}

    fn system_timestamp(&self) -> u64 {
        0
    }

    fn open_link(&self, _url: &str, _new_tab: bool) {
        // noop
    }
}

impl BackendSystem for EmptyBackend {
    fn initialize<S, R>(&mut self, _config: WindowConfig) -> Result<Box<InitializeFn<S, R>>, String>
    where
        S: 'static,
        R: FnMut(&mut App, &mut S) -> Result<FrameState, String> + 'static,
    {
        Ok(Box::new(|mut app: App, mut state: S, mut cb: R| {
            // This function should block with a loop or raf in the platform specific backends
            // while !app.closed {
            if let Err(e) = cb(&mut app, &mut state) {
                log::error!("{}", e);
            }
            // }
            Ok(())
        }))
    }

    fn get_graphics_backend(&self) -> Box<dyn DeviceBackend> {
        Box::new(EmptyDeviceBackend::default())
    }

    fn get_audio_backend(&self) -> Rc<RefCell<Box<dyn AudioBackend>>> {
        Rc::new(RefCell::new(Box::new(EmptyAudioBackend::default())))
    }
}

#[derive(Default)]
struct EmptyDeviceBackend {
    id_count: u64,
}

impl DeviceBackend for EmptyDeviceBackend {
    fn api_name(&self) -> &str {
        ""
    }

    fn create_pipeline(
        &mut self,
        _vertex_source: &[u8],
        _fragment_source: &[u8],
        _vertex_attrs: &[VertexAttr],
        _options: PipelineOptions,
    ) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_vertex_buffer(
        &mut self,
        _attrs: &[VertexAttr],
        _step_mode: VertexStepMode,
    ) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_index_buffer(&mut self) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_uniform_buffer(&mut self, _slot: u32, _name: &str) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn set_buffer_data(&mut self, _id: u64, _data: &[u8]) {}

    fn render(&mut self, commands: &[Commands], _target: Option<u64>) {
        commands.iter().for_each(|cmd| log::info!("{:?}", cmd));
    }

    fn clean(&mut self, to_clean: &[ResourceId]) {
        log::info!("{:?}", to_clean);
    }

    fn set_size(&mut self, _width: i32, _height: i32) {}

    fn set_dpi(&mut self, _scale_factor: f64) {}

    fn create_texture(&mut self, _info: &TextureInfo) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn create_render_texture(
        &mut self,
        _texture_id: u64,
        _info: &TextureInfo,
    ) -> Result<u64, String> {
        self.id_count += 1;
        Ok(self.id_count)
    }

    fn update_texture(&mut self, _texture: u64, _opts: &TextureUpdate) -> Result<(), String> {
        Ok(())
    }

    fn read_pixels(
        &mut self,
        _texture: u64,
        _bytes: &mut [u8],
        _opts: &TextureRead,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Default)]
pub struct EmptyAudioBackend {
    id_count: u64,
    volume: f32,
}

impl AudioBackend for EmptyAudioBackend {
    fn set_global_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    fn global_volume(&self) -> f32 {
        self.volume
    }

    fn create_source(&mut self, _bytes: &[u8]) -> Result<u64, String> {
        let id = self.id_count;
        self.id_count += 1;
        Ok(id)
    }

    fn play_sound(&mut self, _source: u64, _repeat: bool) -> Result<u64, String> {
        let id = self.id_count;
        self.id_count += 1;
        Ok(id)
    }

    fn pause(&mut self, sound: u64) {}

    fn resume(&mut self, _sound: u64) {}

    fn stop(&mut self, _sound: u64) {}

    fn is_stopped(&mut self, sound: u64) -> bool {
        false
    }

    fn is_paused(&mut self, sound: u64) -> bool {
        false
    }

    fn set_volume(&mut self, sound: u64, volume: f32) {}

    fn volume(&self, sound: u64) -> f32 {
        0.0
    }

    fn clean(&mut self, sources: &[u64], sounds: &[u64]) {}
}
