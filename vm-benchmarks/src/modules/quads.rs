use vm::{
    commands,
    commands::Source,
    data::{BytesBuffer, Command},
    module::{Module, ModuleState, CLIENT_ID},
};

use vm_math::*;

pub struct QuadsModule {
    camera_matrices: CameraMatrices,
    camera_transform: OthroCameraTransforms,
    quad_transforms: Transforms2D,
    quad_model_matrix: Mat4f,
    quad_mvp_matrix: Mat4f,
}

#[repr(C)]
pub struct TextCommandPayload {
    pub pos: Vec2f,
    pub text: BytesBuffer,
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
            quad_model_matrix: Mat4f::IDENT,
            quad_mvp_matrix: Mat4f::IDENT,
        }
    }
}

impl Module for QuadsModule {
    fn init(&mut self, _: &mut ModuleState) {}

    fn shutdown(&mut self, _: &mut ModuleState) {}

    fn step(&mut self, state: &mut ModuleState) {
        let commands = state.get_commands(Source::Processor);

        self.camera_matrices = create_ortho_camera_matrices(self.camera_transform);

        self.quad_transforms.position = Vec2f::new(
            self.camera_transform.viewport_size.x / 2.,
            self.camera_transform.viewport_size.y / 2.,
        );

        self.quad_transforms.scaling = Vec2f::new(430., 600.);
        self.quad_transforms.rotation -= 0.25 * state.delta_time;

        self.quad_model_matrix = create_2d_model_matrix(self.quad_transforms);
        self.quad_mvp_matrix = self.camera_matrices.mvp_matrix * self.quad_model_matrix;
    }

    fn render(&mut self, state: &mut ModuleState) {
        let commands_bus = &state.commands_bus;

        let color = Vec4f::new(1.0, 1.0, 0.0, 1.0);
        let command_payload = &[BytesBuffer::new(&[color])];
        let command = Command::new(commands::gapi::SET_COLOR_PIPELINE, command_payload);

        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);

        let command_payload = &[BytesBuffer::new(&[self.quad_mvp_matrix])];
        let command = Command::new(commands::gapi::DRAW_CENTERED_QUADS, command_payload);

         commands_bus.push_command(CLIENT_ID, command, Source::GAPI);

        let frame_time = format!("Frame Time: {:?}", state.last_time.elapsed());

        let command_payload = &[
            /* font id */ // TODO
            /* font size */ BytesBuffer::new::<u32>(&[12]),
            /* text position */ BytesBuffer::new(&[Vec2f::new(100., 100.)]),
            /* text */ BytesBuffer::from_string(&frame_time),
        ];
        let command = Command::new(commands::gapi::DRAW_TEXTS, command_payload);

        commands_bus.push_command(CLIENT_ID, command, Source::GAPI);
    }
}
