use vm::{
    commands,
    commands::Source,
    data::{BytesBuffer, Command},
    gapi,
    module::{Module, ModuleState, CLIENT_ID},
};

use vm_buffers::IntoVMBuffers;
use vm_math::*;

pub struct QuadsModule {
    camera_matrices: CameraMatrices,
    camera_transform: OthroCameraTransforms,
    quad_transforms: Transforms2D,
    text_transforms: Transforms2D,
    quad_model_matrix: Mat4f,
    quad_mvp_matrix: Mat4f,
    boundary_mvp_matrix: Mat4f,
    text_mvp_matrix: Mat4f,
}

impl QuadsModule {
    pub fn new() -> Self {
        QuadsModule {
            camera_matrices: CameraMatrices::default(),
            camera_transform: OthroCameraTransforms {
                viewport_size: Vec2f::new(1024., 768.),
                position: Vec2f::ZERO,
                zoom: 1.,
            },
            quad_transforms: Transforms2D {
                position: Vec2f::ZERO,
                scaling: Vec2f::new(1., 1.),
                rotation: 0.,
            },
            text_transforms: Transforms2D {
                position: Vec2f::ZERO,
                scaling: Vec2f::new(1., 1.),
                rotation: 0.,
            },
            quad_model_matrix: Mat4f::IDENT,
            quad_mvp_matrix: Mat4f::IDENT,
            boundary_mvp_matrix: Mat4f::IDENT,
            text_mvp_matrix: Mat4f::IDENT,
        }
    }
}

impl QuadsModule {
    fn update_camera(&mut self) {
        self.camera_matrices = create_ortho_camera_matrices(self.camera_transform);
    }

    fn update_quad(&mut self, state: &mut ModuleState) {
        self.quad_transforms.position = Vec2f::new(
            self.camera_transform.viewport_size.x / 2.,
            self.camera_transform.viewport_size.y / 2.,
        );

        self.quad_transforms.scaling = Vec2f::new(430., 600.);
        self.quad_transforms.rotation -= 0.25 * state.delta_time;

        self.quad_model_matrix = create_2d_model_matrix(self.quad_transforms);
        self.quad_mvp_matrix = self.camera_matrices.mvp_matrix * self.quad_model_matrix;
    }

    fn update_text(&mut self) {
        self.text_transforms.position =
            Vec2f::new(10.0, self.camera_transform.viewport_size.y - 24.);

        self.text_transforms.scaling = Vec2f::new(1., 1.);

        let model_matrix = create_2d_model_matrix(self.text_transforms);
        self.text_mvp_matrix = self.camera_matrices.mvp_matrix * model_matrix;
    }

    fn update_text_boundary(&mut self, boundary: &Vec2f) {
        let transforms = Transforms2D {
            position: Vec2f::new(10.0, self.camera_transform.viewport_size.y - 24.),
            scaling: Vec2f::new(boundary.x, boundary.y),
            rotation: 0.,
        };

        let model_matrix = create_2d_model_matrix(transforms);
        self.boundary_mvp_matrix = self.camera_matrices.mvp_matrix * model_matrix;
    }
}

impl Module for QuadsModule {
    fn id(&self) -> &'static str {
        "tech.paws.benchmark.quads"
    }

    fn init(&mut self, _: &mut ModuleState) {}

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, state: &mut ModuleState) {
        // let commands = state.get_commands(Source::Processor);

        // unsafe {
        //     let ccommand = &*(commands.commands);

        //     if ccommand.count == 1 {
        //         let payload = &*(ccommand.payload);
        //         let vec = &*(payload.base as *const Vec2f);
        //         self.update_text_boundary(vec);
        //     }
        // }

        self.update_camera();
        self.update_quad(state);
        self.update_text();
    }

    fn render(&mut self, state: &mut ModuleState) {
        let gapi_context = gapi::GApiContext {
            address: CLIENT_ID,
            commands_bus: &mut state.commands_bus,
        };

        gapi::set_color_pipeline(&gapi_context, Vec4f::new(1.0, 1.0, 0.0, 1.0));
        gapi::draw_centered_quads(&gapi_context, &[self.quad_mvp_matrix]);
        gapi::set_color_pipeline(&gapi_context, Vec4f::new(0.0, 0.5, 0.5, 1.0));
        gapi::draw_quads(&gapi_context, &[self.boundary_mvp_matrix]);
        gapi::set_texture_pipeline(&gapi_context, 0);

        let frame_time = format!("Frame Time: {:?}", state.last_time.elapsed());
        let text = gapi::TextData {
            font_id: 0,
            font_size: 14,
            mvp_matrix: self.text_mvp_matrix,
            text: frame_time,
        };

        gapi::draw_texts(&gapi_context, &[text]);
    }
}
