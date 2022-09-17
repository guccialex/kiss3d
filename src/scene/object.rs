//! Data structure of a scene node.

use crate::camera::Camera;
use crate::light::Light;
use crate::resource::{Material, Mesh, Texture, TextureManager};
use na::{Isometry3, Point2, Point3, Vector3};
use std::any::Any;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[path = "../error.rs"]
mod error;

/// Set of data identifying a scene node.
pub struct ObjectData {
    material: Rc<RefCell<Box<dyn Material + 'static>>>,
    texture: Rc<Texture>,
    color: Point3<f32>,
    lines_color: Option<Point3<f32>>,
    wlines: f32,
    wpoints: f32,
    draw_surface: bool,
    cull: bool,
    user_data: Box<dyn Any + 'static>,
}

impl ObjectData {

    /// Creates a deep copy of this object without copying user data
    pub fn try_clone(&self) -> Option<ObjectData> {
        let user_data = ();
        Some(ObjectData {
            material: self.material.clone(),
            texture: self.texture.clone(),
            color: self.color,
            lines_color: self.lines_color,
            wlines: self.wlines,
            wpoints: self.wpoints,
            draw_surface: self.draw_surface,
            cull: self.cull,
            user_data: Box::new(user_data),
        })
    }

    /// The texture of this object.
    #[inline]
    pub fn texture(&self) -> &Rc<Texture> {
        &self.texture
    }

    /// The color of this object.
    #[inline]
    pub fn color(&self) -> &Point3<f32> {
        &self.color
    }

    /// The width of the lines draw for this object.
    #[inline]
    pub fn lines_width(&self) -> f32 {
        self.wlines
    }

    /// The width of the lines draw for this object.
    #[inline]
    pub fn lines_color(&self) -> Option<&Point3<f32>> {
        self.lines_color.as_ref()
    }

    /// The size of the points draw for this object.
    #[inline]
    pub fn points_size(&self) -> f32 {
        self.wpoints
    }

    /// Whether this object has its surface rendered or not.
    #[inline]
    pub fn surface_rendering_active(&self) -> bool {
        self.draw_surface
    }

    /// Whether this object uses backface culling or not.
    #[inline]
    pub fn backface_culling_enabled(&self) -> bool {
        self.cull
    }

    /// An user-defined data.
    ///
    /// Use dynamic typing capabilities of the `Any` type to recover the actual data.
    #[inline]
    pub fn user_data(&self) -> &dyn Any {
        &*self.user_data
    }
}

/// A 3d objects on the scene.
///
/// This is the only interface to manipulate the object position, color, vertices and texture.
pub struct Object {
    // FIXME: should Mesh and Object be merged?
    // (thus removing the need of ObjectData at all.)
    data: ObjectData,
    mesh: Rc<RefCell<Mesh>>,
}

impl Object {
    #[doc(hidden)]
    pub fn new(
        mesh: Rc<RefCell<Mesh>>,
        r: f32,
        g: f32,
        b: f32,
        texture: Rc<Texture>,
        material: Rc<RefCell<Box<dyn Material + 'static>>>,
    ) -> Object {
        let user_data = ();
        let data = ObjectData {
            color: Point3::new(r, g, b),
            lines_color: None,
            texture,
            wlines: 0.0,
            wpoints: 0.0,
            draw_surface: true,
            cull: true,
            material,
            user_data: Box::new(user_data),
        };

        Object { data, mesh }
    }

    /// Creates a deep copy of this object without copying user data
    pub fn try_clone(&self) -> Option<Object> {
        let data = self.data.try_clone()?;
        let mesh = self.mesh.borrow().try_clone(false)?;
        let mesh = Rc::new(RefCell::new(mesh));
        Some(Object { data, mesh })
    }

    #[doc(hidden)]
    pub fn render(
        &self,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        pass: usize,
        camera: &mut dyn Camera,
        light: &Light,
    ) {
        self.data.material.borrow_mut().render(
            pass,
            transform,
            scale,
            camera,
            light,
            &self.data,
            &mut *self.mesh.borrow_mut(),
        );
    }

    /// Gets the data of this object.
    #[inline]
    pub fn data(&self) -> &ObjectData {
        &self.data
    }

    /// Gets the data of this object.
    #[inline]
    pub fn data_mut(&mut self) -> &mut ObjectData {
        &mut self.data
    }

    /// Enables or disables backface culling for this object.
    #[inline]
    pub fn enable_backface_culling(&mut self, active: bool) {
        self.data.cull = active;
    }

    /// Attaches user-defined data to this object.
    #[inline]
    pub fn set_user_data(&mut self, user_data: Box<dyn Any + 'static>) {
        self.data.user_data = user_data;
    }

    /// Gets the material of this object.
    #[inline]
    pub fn material(&self) -> Rc<RefCell<Box<dyn Material + 'static>>> {
        self.data.material.clone()
    }

    /// Sets the material of this object.
    #[inline]
    pub fn set_material(&mut self, material: Rc<RefCell<Box<dyn Material + 'static>>>) {
        self.data.material = material;
    }

    /// Sets the width of the lines drawn for this object.
    #[inline]
    pub fn set_lines_width(&mut self, width: f32) {
        self.data.wlines = width
    }

    /// Returns the width of the lines drawn for this object.
    #[inline]
    pub fn lines_width(&self) -> f32 {
        self.data.wlines
    }

    /// Sets the width of the lines drawn for this object.
    #[inline]
    pub fn set_lines_color(&mut self, color: Option<Point3<f32>>) {
        self.data.lines_color = color
    }

    /// Returns the width of the lines drawn for this object.
    #[inline]
    pub fn lines_color(&self) -> Option<Point3<f32>> {
        self.data.lines_color
    }

    /// Sets the size of the points drawn for this object.
    #[inline]
    pub fn set_points_size(&mut self, size: f32) {
        self.data.wpoints = size
    }

    /// Returns the size of the points drawn for this object.
    #[inline]
    pub fn points_size(&self) -> f32 {
        self.data.wpoints
    }

    /// Activate or deactivate the rendering of this object surface.
    #[inline]
    pub fn set_surface_rendering_activation(&mut self, active: bool) {
        self.data.draw_surface = active
    }

    /// Activate or deactivate the rendering of this object surface.
    #[inline]
    pub fn surface_rendering_activation(&self) -> bool {
        self.data.draw_surface
    }

    /// This object's mesh.
    #[inline]
    pub fn mesh(&self) -> &Rc<RefCell<Mesh>> {
        &self.mesh
    }

    /// Mutably access the object's vertices.
    #[inline(always)]
    pub fn modify_vertices<F: FnMut(&mut Vec<Point3<f32>>)>(&mut self, f: &mut F) {
        let bmesh = self.mesh.borrow_mut();
        let _ = bmesh
            .coords()
            .write()
            .unwrap()
            .data_mut()
            .as_mut()
            .map(|coords| f(coords));
    }

    /// Access the object's vertices.
    #[inline(always)]
    pub fn read_vertices<F: FnMut(&[Point3<f32>])>(&self, f: &mut F) {
        let bmesh = self.mesh.borrow();
        let _ = bmesh
            .coords()
            .read()
            .unwrap()
            .data()
            .as_ref()
            .map(|coords| f(&coords[..]));
    }

    /// Recomputes the normals of this object's mesh.
    #[inline]
    pub fn recompute_normals(&mut self) {
        self.mesh.borrow_mut().recompute_normals();
    }

    /// Mutably access the object's normals.
    #[inline(always)]
    pub fn modify_normals<F: FnMut(&mut Vec<Vector3<f32>>)>(&mut self, f: &mut F) {
        let bmesh = self.mesh.borrow_mut();
        let _ = bmesh
            .normals()
            .write()
            .unwrap()
            .data_mut()
            .as_mut()
            .map(|normals| f(normals));
    }

    /// Access the object's normals.
    #[inline(always)]
    pub fn read_normals<F: FnMut(&[Vector3<f32>])>(&self, f: &mut F) {
        let bmesh = self.mesh.borrow();
        let _ = bmesh
            .normals()
            .read()
            .unwrap()
            .data()
            .as_ref()
            .map(|normals| f(&normals[..]));
    }

    /// Mutably access the object's faces.
    #[inline(always)]
    pub fn modify_faces<F: FnMut(&mut Vec<Point3<u16>>)>(&mut self, f: &mut F) {
        let bmesh = self.mesh.borrow_mut();
        let _ = bmesh
            .faces()
            .write()
            .unwrap()
            .data_mut()
            .as_mut()
            .map(|faces| f(faces));
    }

    /// Access the object's faces.
    #[inline(always)]
    pub fn read_faces<F: FnMut(&[Point3<u16>])>(&self, f: &mut F) {
        let bmesh = self.mesh.borrow();
        let _ = bmesh
            .faces()
            .read()
            .unwrap()
            .data()
            .as_ref()
            .map(|faces| f(&faces[..]));
    }

    /// Mutably access the object's texture coordinates.
    #[inline(always)]
    pub fn modify_uvs<F: FnMut(&mut Vec<Point2<f32>>)>(&mut self, f: &mut F) {
        let bmesh = self.mesh.borrow_mut();
        let _ = bmesh
            .uvs()
            .write()
            .unwrap()
            .data_mut()
            .as_mut()
            .map(|uvs| f(uvs));
    }

    /// Access the object's texture coordinates.
    #[inline(always)]
    pub fn read_uvs<F: FnMut(&[Point2<f32>])>(&self, f: &mut F) {
        let bmesh = self.mesh.borrow();
        let _ = bmesh
            .uvs()
            .read()
            .unwrap()
            .data()
            .as_ref()
            .map(|uvs| f(&uvs[..]));
    }

    /// Sets the color of the object.
    ///
    /// Colors components must be on the range `[0.0, 1.0]`.
    #[inline]
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.data.color.x = r;
        self.data.color.y = g;
        self.data.color.z = b;
    }

    /// Sets the texture of the object.
    ///
    /// The texture is loaded from a file and registered by the global `TextureManager`.
    ///
    /// # Arguments
    ///   * `path` - relative path of the texture on the disk
    #[inline]
    pub fn set_texture_from_file(&mut self, path: &Path, name: &str) {
        let texture = TextureManager::get_global_manager(|tm| tm.add(path, name));

        self.set_texture(texture)
    }

    /// Sets the texture of the object.
    ///
    /// The texture must already have been registered as `name`.
    #[inline]
    pub fn set_texture_with_name(&mut self, name: &str) {
        let texture = TextureManager::get_global_manager(|tm| {
            tm.get(name).unwrap_or_else(|| {
                panic!("Invalid attempt to use the unregistered texture: {}", name)
            })
        });

        self.set_texture(texture)
    }

    /// Sets the texture of the object.
    #[inline]
    pub fn set_texture(&mut self, texture: Rc<Texture>) {
        self.data.texture = texture
    }
}
