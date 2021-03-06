/* -----------------------------------------------------------------------------
	MithrilEngine Framework (MEF)

	Copyright (c) 2021-2022, daigennki (@daigennki)
----------------------------------------------------------------------------- */
pub mod ui;
pub mod mesh;
pub mod camera;

use std::sync::Arc;
use glam::*;
use vulkano::buffer::{ /*ImmutableBuffer,*/ BufferUsage, cpu_access::CpuAccessibleBuffer };
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;
use vulkano::descriptor_set::WriteDescriptorSet;
use crate::render::RenderContext;

pub struct Transform
{
	// TODO: parent-child relationship
	// TODO: maybe we should use immutable buffers but only for static objects...
	buf: Arc<CpuAccessibleBuffer<[f32]>>,
	descriptor_set: Arc<PersistentDescriptorSet>,
	pos: Vec3,
	scale: Vec3,
	rot: Vec3	// rotation on X, Y, and Z axes
}
impl Transform
{
	pub fn new(render_ctx: &mut RenderContext, pos: Vec3, scale: Vec3) -> Result<Transform, Box<dyn std::error::Error>>
	{
		let rot = Vec3::ZERO;
		let rot_quat = Quat::from_euler(EulerRot::XYZ, rot.x, rot.y, rot.z);
		let transform_mat = Mat4::from_scale_rotation_translation(
			scale,
			rot_quat,
			pos
		);
		let buf = render_ctx.new_cpu_buffer(transform_mat.to_cols_array(), BufferUsage::uniform_buffer())?;

		Ok(Transform{ 
			buf: buf.clone(),
			descriptor_set: render_ctx.new_descriptor_set("World", 0, [
				WriteDescriptorSet::buffer(0, buf)
			])?, 
			pos: pos, 
			scale: scale,
			rot: rot
		})
	}

	fn update_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>>
	{
		let transform_mat = Mat4::from_scale_rotation_translation(
			self.scale,
			Quat::from_euler(EulerRot::XYZ, self.rot.x, self.rot.y, self.rot.z),
			self.pos
		);

		self.buf.write()?.clone_from_slice(&transform_mat.to_cols_array());
			
		Ok(())
	}

	pub fn set_pos(&mut self, pos: Vec3) -> Result<(), Box<dyn std::error::Error>>
	{
		self.pos = pos;
		self.update_buffer()
	}

	pub fn bind_descriptor_set(&self, render_ctx: &mut RenderContext) -> Result<(), crate::render::PipelineNotLoaded>
	{
		render_ctx.bind_descriptor_set(0, self.descriptor_set.clone())
	}
}

/// Convenience function: create a tuple of `Transform` and `Mesh` to display a simple triangle.
pub fn new_triangle(render_ctx: &mut RenderContext, pos: Vec3, scale: Vec3, color: Vec4)
	-> Result<(Transform, mesh::Mesh), Box<dyn std::error::Error>>
{
	let tri_transform = Transform::new(render_ctx, pos, scale)?;
	let tri_mesh = mesh::Mesh::new(render_ctx, color)?;

	Ok((tri_transform, tri_mesh))
}

