use core::libc::{c_void, c_char, c_int, ptrdiff_t};

static lua_multret : c_int = -1;

type lua_State = *c_void;
type lua_Integer = ptrdiff_t;

#[link_name="lua5.1"]
extern mod __ffi__ {
	fn lua_newstate(alloc: *u8, userdata: *c_void) -> lua_State;
	fn lua_call(state: lua_State, nargs: c_int, nresults: c_int);
	fn lua_pcall(state: lua_State, nargs: c_int, 
				 nresults: c_int, errfunc: c_int) -> c_int;
	fn lua_getfield(state: lua_State, index: c_int, name: *c_char);

	fn lua_pushinteger(state: lua_State, n: lua_Integer);

	fn lua_tointeger(state: lua_State, index: c_int) -> lua_Integer;

	fn luaL_loadstring(state: lua_State, string: *c_char) -> c_int;
}

pub struct LuaState { priv opaque: lua_State }

pub enum NResults {
	AllResults,
	SomeResults(int)
}

impl NResults {
	fn as_c_int(self) -> c_int {
		match self {
			AllResults => lua_multret,
			SomeResults(x) => x as c_int
		}
	}
}

impl LuaState {
	pub fn new(allocator: *u8, userdata: *c_void) -> LuaState {
		unsafe {
			return LuaState {
				opaque: __ffi__::lua_newstate(allocator, userdata)
			};
		}
	}

	pub fn getglobal(&self, name: &str) {
		unsafe {
			str::as_c_str(name, |s| __ffi__::lua_getfield(self.opaque, -10002, s));
		}
	}

	pub fn L_dostring(&self, string: &str) -> c_int {
		unsafe {
			let mut result = 
				str::as_c_str(string, |s| __ffi__::luaL_loadstring(self.opaque, s));

			if result == 0 {
				result = __ffi__::lua_pcall(self.opaque, 0, lua_multret, 0);
			}

			return result;
		}
	}

	pub fn call(&self, nargs: int, nresults: NResults)
		{ unsafe { __ffi__::lua_call(self.opaque, nargs as c_int, nresults.as_c_int()) } }

	pub fn push(&self, n: int) 
		{ unsafe { __ffi__::lua_pushinteger(self.opaque, n as lua_Integer) } }

	pub fn tointeger(&self, index: int) -> int
		{ unsafe { __ffi__::lua_tointeger(self.opaque, index as c_int) as int } }
}