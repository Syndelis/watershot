use smithay_client_toolkit::{
    delegate_layer,
    reexports::client::{Connection, QueueHandle},
    shell::wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
};

use crate::{runtime_data::RuntimeData, types::MonitorIdentification};

delegate_layer!(RuntimeData);

impl LayerShellHandler for RuntimeData {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {}

    fn configure(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        _configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        let _ = self.themed_pointer.as_ref().unwrap().set_cursor(
            conn,
            "crosshair",
            self.shm_state.wl_shm(),
            &self.pointer_surface,
            1,
        );

        log::info!("{:?}", _configure);

        let monitor = self
            .monitors
            .iter_mut()
            .find(|window| window.layer == *layer)
            .unwrap();
        let cap = monitor.surface.get_capabilities(&self.adapter);

        monitor.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: (monitor.rect.width * monitor.scale_factor) as u32,
                height: (monitor.rect.height * monitor.scale_factor) as u32,
                present_mode: wgpu::PresentMode::Mailbox,
                alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
            },
        );

        log::info!("{:?}", cap.formats);

        self.draw(MonitorIdentification::Layer(layer.clone()), qh);
    }
}
