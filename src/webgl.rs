use common::*;
use glenum::*;
use std::ops::Deref;

//use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element,WebGlProgram, WebGlRenderingContext, 
WebGlShader,HtmlCanvasElement,WebGlBuffer};

pub type Reference = WebGlRenderingContext;

#[derive(Debug, Clone)]
pub struct GLContext {
    pub reference: Reference,
    pub is_webgl2: bool,
}

pub type WebGLContext<'a> = &'a HtmlCanvasElement;

impl WebGLRenderingContext {
    pub fn new(canvas: &WebGLContext) -> WebGLRenderingContext {
        WebGLRenderingContext {
            common: GLContext::new(&canvas),
        }
    }
}

impl GLContext {
    #[inline]
    pub fn log<T: Into<JsValue>>(&self, _msg: T) {
        // js!{ console.log(@{msg.into()})};
        web_sys::console::log_1(&_msg.into());
    }

    pub fn print<T: Into<JsValue>>(msg: T) {
        web_sys::console::log_1(&msg.into());
    }

    pub fn new<'a>(canvas: &WebGLContext) -> GLContext {
        let context = canvas
           .get_context("webgl")
           .unwrap()
           .unwrap()
           .dyn_into::<WebGlRenderingContext>().unwrap();
        GLContext {
            reference: context,
            is_webgl2: true,
        }
    }

    pub fn create_buffer(&self) -> WebGLBuffer<WebGlBuffer> {
        self.log("create_buffer");
        let k:&WebGlRenderingContext = &self.reference;
        WebGLBuffer(k.create_buffer().unwrap())
    }

    pub fn delete_buffer(&self, buffer: &WebGLBuffer<WebGlBuffer>) {
        self.log("delete_buffer");
        let k:&WebGlRenderingContext = &self.reference;
        let b2:&WebGlBuffer = buffer.deref();
        k.delete_buffer(Some(b2));
    }

    pub fn buffer_data(&self, kind: BufferKind, data: &mut [u8], draw: DrawMode) {
        self.log("buffer_data");
        let k:&WebGlRenderingContext = &self.reference;
        k.buffer_data_with_u8_array(kind as u32, data,draw as u32 )
    }

    pub fn bind_buffer(&self, kind: BufferKind, buffer: &WebGLBuffer<WebGlBuffer>) {
        self.log("bind_buffer");
        let k:&WebGlRenderingContext = &self.reference;
        k.bind_buffer(kind as u32,Some(buffer.deref()));
    }

    pub fn unbind_buffer(&self, kind: BufferKind) {
        self.log("unbind_buffer");
        let k:&WebGlRenderingContext = &self.reference;
        k.bind_buffer(kind as u32,None);
    }

    pub fn create_shader(&self, kind: ShaderKind) -> WebGLShader<WebGlShader> {
        self.log("create_shader");
        let k:&WebGlRenderingContext = &self.reference;
        let value = k.create_shader(kind as u32).unwrap();
        WebGLShader(value)
    }
/*
    pub fn shader_source(&self, shader: &WebGLShader, code: &str) {
        self.log("shader_source");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            var shader = Module.gl.get(@{shader.deref()});
            ctx.shaderSource(shader,@{ code })
        };
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        self.log("compile_shader");
        js! {
            var ctx = Module.gl.get(@{&self.reference});
            var shader = Module.gl.get(@{shader.deref()});
            ctx.compileShader(shader);

            var compiled = ctx.getShaderParameter(shader, 0x8B81);
            if (!compiled ) {
                console.log("ERROR in shader compilation:");
                console.log( ctx.getShaderInfoLog(shader));
            }
        };
    }

    pub fn create_program(&self) -> WebGLProgram {
        self.log("create_program");
        let value = js! {
            var ctx = Module.gl.get(@{&self.reference});
            var h = {};
            h.prog = ctx.createProgram();
            h.uniform_names = {};
            return Module.gl.add(h);
        };
        WebGLProgram(value.try_into().unwrap())
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        self.log("link_program");
        js! {
            var ctx = Module.gl.get(@{&self.reference});
            var h = Module.gl.get(@{program.deref()});
            ctx.linkProgram(h.prog);
            var result=ctx.getProgramParameter(h.prog, ctx.LINK_STATUS);
            if (! result) {
                console.log("ERROR while linking program :");
                console.log(ctx.getProgramInfoLog(h.prog));
            }
        };
    }

    pub fn use_program(&self, program: &WebGLProgram) {
        self.log("use_program");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            var h = Module.gl.get(@{program.deref()});
            ctx.useProgram(h.prog)
        };
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        self.log("attach_shader");
        js! {
            var ctx = Module.gl.get(@{&self.reference});
            var h = Module.gl.get(@{program.deref()});
            var shader = Module.gl.get(@{shader.deref()});
            ctx.attachShader(h.prog, shader)
        };
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, name: &str, loc: u32) {
        self.log("bind_attrib_location");
        js! {
            @(no_return)

            var ctx = Module.gl.get(@{&self.reference});
            var h = Module.gl.get(@{program.deref()});
            ctx.bindAttribLocation(h.prog,@{loc}, @{name});
        };
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> Option<u32> {
        self.log("get_attrib_location");
        let value = js! {
            var ctx = Module.gl.get(@{&self.reference});
            var h = Module.gl.get(@{program.deref()});
            var r = ctx.getAttribLocation(h.prog,@{name});
            return r >= 0 ? r : null;
        };
        value.try_into().ok() as _
    }

    pub fn get_uniform_location(
        &self,
        program: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation> {
        self.log("get_uniform_location");
        let value = js! {
            var ctx = Module.gl.get(@{&self.reference});
            var h = Module.gl.get(@{program.deref()});

            var name = @{name};
            var uniform = h.uniform_names[name];
            if(name in h.uniform_names) return h.uniform_names[name];

            uniform = Module.gl.add(ctx.getUniformLocation(h.prog,name));
            h.uniform_names[name] = uniform;

            return uniform;
        };

        value.try_into().ok().map(|uni| WebGLUniformLocation {
            reference: uni,
            name: name.into(),
        })
    }

    pub fn vertex_attrib_pointer(
        &self,
        location: u32,
        size: AttributeSize,
        kind: DataType,
        normalized: bool,
        stride: u32,
        offset: u32,
    ) {
        self.log("vertex_attribute_pointer");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});

            ctx.vertexAttribPointer(@{location},@{size as u16},@{kind as i32},@{normalized},@{stride},@{offset});
        };
    }

    pub fn enable_vertex_attrib_array(&self, location: u32) {
        self.log("enabled_vertex_attrib_array");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.enableVertexAttribArray(@{location})
        };
    }
*/
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.log("clear_color");
        let k:&WebGlRenderingContext = &self.reference;
        k.clear_color(r,g,b,a);
    }
/*
    pub fn enable(&self, flag: i32) {
        self.log("enable");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.enable(@{flag as i32});
        };
    }

    pub fn disable(&self, flag: i32) {
        self.log("disable");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.disable(@{flag as i32});
        };
    }

    pub fn cull_face(&self, flag: Culling) {
        self.log("cull_face");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.cullFace(@{flag as i32});
        };
    }

    pub fn depth_mask(&self, b: bool) {
        self.log("depth_mask");

        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.depthMask(@{b});
        }
    }

    pub fn depth_func(&self, d: DepthTest) {
        self.log("depth_func");

        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.depthFunc(@{d as i32});
        }
    }

    pub fn clear_depth(&self, value: f32) {
        self.log("clear_depth");

        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.clearDepth(@{value});
        }
    }
*/
    pub fn clear(&self, bit: BufferBit) {
        self.log("clear");
        let k:&WebGlRenderingContext = &self.reference;
        k.clear(bit as u32);
    }
/*
    pub fn viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        self.log("viewport");
        let params = js! { return [@{x},@{y},@{width},@{height}] };
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            var p = @{params};
            ctx.viewport(p[0],p[1],p[2],p[3]);
        };
    }

    pub fn draw_elements(&self, mode: Primitives, count: usize, kind: DataType, offset: u32) {
        self.log("draw_elemnts");
        self.reference.draw_elements_with_i32(mode as i32, count as i32, kind as i32, offset as i32)
    }

    pub fn draw_arrays(&self, mode: Primitives, count: usize) {
        self.log("draw_arrays");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.drawArrays(@{mode as i32},0,@{count as i32});
        };
    }

    pub fn read_pixels(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: PixelFormat,
        kind: PixelType,
        data: &mut [u8],
    ) {
        self.log("read_pixels");
        let data_len = data.len();

        let pixels = js!{
            var ctx = Module.gl.get(@{&self.reference});

            var pixelValues = new Uint8Array(@{data_len as u32});
            ctx.readPixels(
                @{x as i32},
                @{y as i32},
                @{width as i32},
                @{height as i32},
                @{format as u32},
                @{kind as u32},
                pixelValues,
            );

            return pixelValues;
        };

        let pixels: TypedArray<u8> = pixels.try_into().unwrap();
        let pixels_arr: Vec<_> = pixels.into();

        data.clone_from_slice(&pixels_arr);
    }

    pub fn pixel_storei(&self, storage: PixelStorageMode, value: i32) {
        self.log("pixel_storei");
        js!{
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.pixelStorei(@{storage as i32},@{value});
        }
    }

    pub fn generate_mipmap(&self) {
        self.log("generate_mipmap");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.generateMipmap(ctx.TEXTURE_2D);
        }
    }

    pub fn generate_mipmap_cube(&self) {
        self.log("generate_mipmap_cube");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{&self.reference});
            ctx.generateMipmap(ctx.TEXTURE_CUBE_MAP);
        }
    }

    pub fn tex_image2d(
        &self,
        target: TextureBindPoint,
        level: u8,
        width: u16,
        height: u16,
        format: PixelFormat,
        kind: PixelType,
        pixels: &[u8],
    ) {
        self.log("tex_img2d");
        let params1 = js! { return [@{target as u32},@{level as u32},@{format as u32}] };
        let params2 =
            js! { return [@{width as u32},@{height as u32},@{format as u32},@{kind as u32}] };

        // TODO: It is a strange bug !!!
        // According https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
        // the format arg should be equal to internal format arg
        // however, only DEPTH_COMPONENT16 works but not DEPTH_COMPONENT

        let is_depth = match format {
            PixelFormat::DepthComponent => true,
            _ => false,
        };

        if pixels.len() > 0 {
            js!{
                var p = @{params1}.concat(@{params2});
                var ctx = Module.gl.get(@{&self.reference});

                ctx.texImage2D(p[0],p[1], p[2] ,p[3],p[4],0,p[2],p[6],@{TypedArray::from(pixels)});
            };
        } else {
            js!{
                var p = @{params1}.concat(@{params2});
                var ctx = Module.gl.get(@{&self.reference});

                var internal_fmt =  @{format as u32};
                var fmt = internal_fmt;
                if ( @{is_depth}) {
                    internal_fmt =  ctx.DEPTH_COMPONENT16;
                }

                ctx.texImage2D(p[0],p[1], internal_fmt ,p[3],p[4],0, fmt ,p[6],null);
            };
        }
    }

    pub fn tex_sub_image2d(
        &self,
        target: TextureBindPoint,
        level: u8,
        xoffset: u16,
        yoffset: u16,
        width: u16,
        height: u16,
        format: PixelFormat,
        kind: PixelType,
        pixels: &[u8],
    ) {
        self.log("sub_tex_img2d");
        let params1 =
            js! { return [@{target as u32},@{level as u32},@{xoffset as u32},@{yoffset as u32}] };
        let params2 =
            js! { return [@{width as u32},@{height as u32},@{format as u32},@{kind as u32}] };
        js!{
            var p = @{params1}.concat(@{params2});
            var ctx = Module.gl.get(@{&self.reference});
            ctx.texSubImage2D(p[0],p[1],p[2],p[3],p[4],p[5],p[6],p[7],@{TypedArray::from(pixels)});
        };
    }

    pub fn compressed_tex_image2d(
        &self,
        target: TextureBindPoint,
        level: u8,
        compression: TextureCompression,
        width: u16,
        height: u16,
        data: &[u8],
    ) {
        self.log("compressed_tex_img2d");
        let params =
            js! { return [@{target as u32},@{level as u32},@{width as u32},@{height as u32}] };
        // for some reason this needs to be called otherwise invalid format error, extension initialization?
        js! {
            var ctx = Module.gl.get(@{&self.reference});

            // for some reason this needs to be called otherwise invalid format error, extension initialization?
            (ctx.getExtension("WEBGL_compressed_texture_s3tc") ||
                ctx.getExtension("MOZ_WEBGL_compressed_texture_s3tc") ||
                ctx.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc"));

            var p = @{params};

            ctx.compressedTexImage2D(
                p[0],
                p[1],
                @{compression as u16},
                p[2],
                p[3],
                0,
                @{TypedArray::from(data)}
            );

            return 0;
        }
        self.log("compressed_tex_img2d end");
    }

    ///
    pub fn create_texture(&self) -> WebGLTexture {
        self.log("create_tex");
        let handle = js!{
            var ctx = Module.gl.get(@{&self.reference});
            return Module.gl.add(ctx.createTexture()) ;
        };
        WebGLTexture(handle.try_into().unwrap())
    }

    pub fn delete_texture(&self, texture: &WebGLTexture) {
        self.log("delete_tex");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            var tex = Module.gl.get(@{&texture.0});
            ctx.deleteTexture(tex);
            Module.gl.remove(tex);
        }
    }

    pub fn active_texture(&self, active: u32) {
        self.log("active_texture");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.activeTexture(ctx.TEXTURE0 + @{active})
        }
    }

    pub fn bind_texture(&self, texture: &WebGLTexture) {
        self.log("bind_tex");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            var tex = Module.gl.get(@{&texture.0});
            ctx.bindTexture(@{TextureKind::Texture2d as u32 }, tex)
        }
    }

    pub fn unbind_texture(&self) {
        self.log("unbind_tex");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.bindTexture(@{TextureKind::Texture2d as u32 },null)
        }
    }

    pub fn bind_texture_cube(&self, texture: &WebGLTexture) {
        self.log("bind_tex_cube");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            var tex = Module.gl.get(@{&texture.0});
            ctx.bindTexture(@{TextureKind::TextureCubeMap as u32 }, tex)
        }
    }

    pub fn unbind_texture_cube(&self) {
        self.log("unbind_tex_cube");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.bindTexture(@{TextureKind::TextureCubeMap as u32 },null)
        }
    }

    pub fn blend_equation(&self, eq: BlendEquation) {
        self.log("blend_equation");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.blendEquation(@{eq as u32});
        }
    }

    pub fn blend_func(&self, b1: BlendMode, b2: BlendMode) {
        self.log("blend_func");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.blendFunc(@{b1 as u32},@{b2 as u32})
        }
    }

    pub fn blend_color(&self, r: f32, g: f32, b: f32, a: f32) {
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.blendColor(@{r}, @{g}, @{b}, @{a});
        }
    }

    pub fn uniform_matrix_4fv(&self, location: &WebGLUniformLocation, value: &[[f32; 4]; 4]) {
        self.log("uniform_matrix_4fv");
        use std::mem;
        let array = unsafe { mem::transmute::<&[[f32; 4]; 4], &[f32; 16]>(value) as &[f32] };

        unsafe {
            __js_uniform4vf(
                self.reference,
                *location.deref(),
                array[0],
                array[1],
                array[2],
                array[3],
                array[4],
                array[5],
                array[6],
                array[7],
                array[8],
                array[9],
                array[10],
                array[11],
                array[12],
                array[13],
                array[14],
                array[15],
                r###"
            var ctx = Module.gl.get($0);
            var loc = Module.gl.get($1);
            var m = Module.gl.matrix4x4;
            m[0] = $2;
            m[1] = $3;
            m[2] = $4;
            m[3] = $5;
            m[4] = $6;
            m[5] = $7;
            m[6] = $8;
            m[7] = $9;
            m[8] = $10;
            m[9] = $11;
            m[10] = $12;
            m[11] = $13;
            m[12] = $14;
            m[13] = $15;
            m[14] = $16;
            m[15] = $17;

            return ctx.uniformMatrix4fv(loc,false, m);
        "### as *const _ as *const u8,
            );
        }
    }

    pub fn uniform_matrix_3fv(&self, location: &WebGLUniformLocation, value: &[[f32; 3]; 3]) {
        self.log("uniform_matrix_3fv");
        use std::mem;
        let array = unsafe { mem::transmute::<&[[f32; 3]; 3], &[f32; 9]>(value) as &[f32] };
        js!{
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});
            ctx.uniformMatrix3fv(loc,false,@{&array})
        }
    }

    pub fn uniform_matrix_2fv(&self, location: &WebGLUniformLocation, value: &[[f32; 2]; 2]) {
        use std::mem;
        let array = unsafe { mem::transmute::<&[[f32; 2]; 2], &[f32; 4]>(value) as &[f32] };
        js!{
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});
            ctx.uniformMatrix2fv(loc,false,@{&array})
        }
    }

    pub fn uniform_1i(&self, location: &WebGLUniformLocation, value: i32) {
        js!{
            @(no_return)
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});
            ctx.uniform1i(loc,@{value})
        }
    }

    pub fn uniform_1f(&self, location: &WebGLUniformLocation, value: f32) {
        js!{
            @(no_return)
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});
            ctx.uniform1f(loc,@{value});
        }
    }

    pub fn uniform_2f(&self, location: &WebGLUniformLocation, value: (f32, f32)) {
        js!{
            @(no_return)
            var p = [@{value.0},@{value.1}];
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});

            ctx.uniform2f(loc,p[0],p[1])
        }
    }

    pub fn uniform_3f(&self, location: &WebGLUniformLocation, value: (f32, f32, f32)) {
        js!{
            @(no_return)
            var p = [@{value.0},@{value.1},@{value.2}];
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});

            ctx.uniform3f(loc,p[0],p[1],p[2])
        }
    }

    pub fn uniform_4f(&self, location: &WebGLUniformLocation, value: (f32, f32, f32, f32)) {
        js!{
            @(no_return)
            var p = [@{value.0},@{value.1},@{value.2},@{value.3}];
            var ctx = Module.gl.get(@{self.reference});
            var loc = Module.gl.get(@{location.reference});

            ctx.uniform4f(loc,p[0],p[1],p[2],p[3])
        }
    }

    pub fn create_vertex_array(&self) -> WebGLVertexArray {
        self.log("create_vertex_array");
        let val = js! {
            var ctx = Module.gl.get(@{self.reference});
            if (ctx.createVertexArray) {
                return Module.gl.add(ctx.createVertexArray());
            } else {
                return 0;
            }
        };
        WebGLVertexArray(val.try_into().unwrap())
    }

    pub fn delete_vertex_array(&self, vao: &WebGLVertexArray) {
        self.log("delete_vertex_array");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{self.reference});
            if (ctx.deleteVertexArray) {
                var vao = Module.gl.get(@{vao.0});
                ctx.deleteVertexArray(vao);
            }
        };
    }

    pub fn bind_vertex_array(&self, vao: &WebGLVertexArray) {
        self.log("bind_vertex_array");
        self.reference.bind_vertex_array(Some(vao));
    }

    pub fn unbind_vertex_array(&self, vao: &WebGLVertexArray) {
        self.log("unbind_vertex_array");
        js! {
            @(no_return)
            var ctx = Module.gl.get(@{self.reference});
            if (ctx.unbindVertexArray) {
                var vao = Module.gl.get(@{vao.0});
                ctx.unbindVertexArray(vao);
            }
        }
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: ShaderParameter) -> i32 {
        let res = js! {
            var h = Module.gl.get(@{program.deref()});
            var ctx = Module.gl.get(@{self.reference});

            return ctx.getProgramParameter(h.prog,@{pname as u32});
        };

        res.try_into().unwrap()
    }

    // pub fn get_active_uniform(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
    //     let res = js! {
    //         var h = Module.gl.get(@{program.deref()});
    //         var ctx = Module.gl.get(@{self.reference});

    //         return ctx.getActiveUniform(h.prog,@{location})
    //     };

    //     let name = js! { return @{&res}.name };
    //     let size = js!{ return @{&res}.size };
    //     let kind = js!{ return @{&res}.type };
    //     let k: u32 = kind.try_into().unwrap();
    //     use std::mem;
    //     WebGLActiveInfo::new(
    //         name.into_string().unwrap(),
    //         size.try_into().unwrap(),
    //         unsafe { mem::transmute::<u16, UniformType>(k as _) },
    //         res.into_reference().unwrap(),
    //     )
    // }

    // pub fn get_active_attrib(&self, program: &WebGLProgram, location: u32) -> WebGLActiveInfo {
    //     let res = js! {
    //         var h = Module.gl.programs[@{program.deref()}];
    //         return @{self.reference}.getActiveAttrib(h.prog,@{location})
    //     };
    //     let name = js! { return @{&res}.name };
    //     let size = js!{ return @{&res}.size };
    //     let kind = js!{ return @{&res}.type };
    //     let k: u32 = kind.try_into().unwrap();
    //     use std::mem;
    //     WebGLActiveInfo::new(
    //         name.into_string().unwrap(),
    //         size.try_into().unwrap(),
    //         unsafe { mem::transmute::<u16, UniformType>(k as _) },
    //         res.into_reference().unwrap(),
    //     )
    // }

    pub fn tex_parameteri(&self, kind: TextureKind, pname: TextureParameter, param: i32) {
        // skip not supported flag in for webgl 1 context
        if !self.is_webgl2 {
            if let TextureParameter::TextureWrapR = pname {
                return;
            }
        }

        js! {
            var ctx = Module.gl.get(@{self.reference});
            return ctx.texParameteri(@{kind as u32},@{pname as u32},@{param})
        };
    }

    pub fn tex_parameterfv(&self, kind: TextureKind, pname: TextureParameter, param: f32) {
        js! {
            var ctx = Module.gl.get(@{self.reference});
            return ctx.texParameterf(@{kind as u32},@{pname as u32},@{param})
        };
    }

    pub fn draw_buffer(&self, buffers: &[ColorBuffer]) {
        self.log("draw_buffer");

        let color_enums: Vec<i32> = buffers.iter().map(|c| *c as i32).collect();

        js! {
            @(no_return)

            var ctx = Module.gl.get(@{self.reference});
            ctx.drawBuffers(@{color_enums});
        };
    }

    pub fn create_framebuffer(&self) -> WebGLFrameBuffer {
        let val = js! {
            var ctx = Module.gl.get(@{self.reference});
            return Module.gl.add(ctx.createFramebuffer());
        };
        WebGLFrameBuffer(val.try_into().unwrap())
    }

    pub fn delete_framebuffer(&self, fb: &WebGLFrameBuffer) {
        js! {
            var ctx = Module.gl.get(@{self.reference});
            var fb = Module.gl.get(@{fb.deref()});
            ctx.deleteFramebuffer(fb);
        }
    }

    pub fn bind_framebuffer(&self, buffer: Buffers, fb: &WebGLFrameBuffer) {
        js! {
            var ctx = Module.gl.get(@{self.reference});
            var fb = Module.gl.get(@{fb.deref()});
            ctx.bindFramebuffer(@{buffer as u32}, fb);
        }
    }

    pub fn framebuffer_texture2d(
        &self,
        target: Buffers,
        attachment: Buffers,
        textarget: TextureBindPoint,
        texture: &WebGLTexture,
        level: i32,
    ) {
        js! {
            var ctx = Module.gl.get(@{self.reference});
            var tex = Module.gl.get(@{&texture.0});
            ctx.framebufferTexture2D(@{target as u32},@{attachment as u32},@{textarget as u32},tex,@{level});
        }
    }

    pub fn unbind_framebuffer(&self, buffer: Buffers) {
        self.log("unbind_framebuffer");
        js!{
            var ctx = Module.gl.get(@{&self.reference});
            ctx.bindFramebuffer(@{buffer as u32},null)
        }
    }
*/
}
