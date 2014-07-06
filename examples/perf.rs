//use std::str;
//use std::vec;
use nanovg::{Ctx, LEFT,RIGHT,TOP,BOTTOM, rgba};

#[repr(i32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum Style {
    FPS,
    MS
}

static CAP:uint = 100;

pub struct PerfGraph {
	pub style: Style,
	pub name: String,
	values: [f32, ..CAP],
	head: uint,
	count: uint,
}



impl PerfGraph
{
	pub fn init(style: Style, name: &str) -> PerfGraph
	{
		PerfGraph {
			style: style,
			name: String::from_str(name),
			values: [0.0, ..CAP],
			head: 0,
			count: 0,
		}
	}

	pub fn update(&mut self, frameTime: f64)
	{
		//fps->head = (fps->head+1) % GRAPH_HISTORY_COUNT;
		//fps->values[fps->head] = frameTime;
		if self.count == CAP { self.head = (self.head + 1) % CAP }
		self.count = if self.count < CAP { self.count + 1 } else { CAP } ;
		self.values[((self.head+self.count) % CAP) as uint] = frameTime as f32;
	}

	pub fn render(&self, vg: &Ctx, x: f32, y: f32)
	{
		let avg = self.get_graph_average();

		let w = 200.0;
		let h = 35.0;

		vg.begin_path();
		vg.rect(x,y, w,h);
		vg.fill_color(rgba(0,0,0,128));
		vg.fill();

		vg.begin_path();
		vg.move_to(x, y+h);
		if self.style == FPS {
			for i in range(0, CAP) { //(i = 0; i < CAP; i++) {
				let mut v = 1.0 / (0.00001 + self.values[(self.head+i) % CAP]);
				if v > 80.0 {v = 80.0;}
				let vx = x + (i as f32 / (CAP-1) as f32) * w;
				let vy = y + h - ((v / 80.0) * h);
				vg.line_to(vx, vy);
			}
		} else {
			for i in range(0, CAP) {
				let mut v = self.values[(self.head+i) % CAP] * 1000.0;
				if v > 20.0 {v = 20.0;}
				let vx = x + (i as f32 / (CAP-1) as f32) * w;
				let vy = y + h - ((v / 20.0) * h);
				vg.line_to(vx, vy);
			}
		}
		vg.line_to(x+w, y+h);
		vg.fill_color(rgba(255,192,0,128));
		vg.fill();

		vg.font_face("sans");

		if self.name.is_empty() {
			vg.font_size(14.0);
			vg.text_align(LEFT|TOP);
			vg.fill_color(rgba(240,240,240,192));
			vg.text(x+3.0,y+1.0, self.name.as_slice());
		}

		if self.style == FPS {
			vg.font_size(18.0);
			vg.text_align(RIGHT|TOP);
			vg.fill_color(rgba(240,240,240,255));
			let txt = format!("{:3.1f} FPS", 1.0 / avg);
			vg.text(x+w-3.0,y+1.0, txt.as_slice());

			vg.font_size(15.0);
			vg.text_align(RIGHT|BOTTOM);
			vg.fill_color(rgba(240,240,240,160));
			let txt = format!("{:3.2f} ms", avg * 1000.0);
			vg.text(x+w-3.0,y+h-1.0, txt.as_slice());
		} else {
			vg.font_size(18.0);
			vg.text_align(RIGHT|TOP);
			vg.fill_color(rgba(240,240,240,255));
			let txt = format!("{:3.2f} ms", avg * 1000.0);
			vg.text(x+w-3.0,y+1.0, txt.as_slice());
		}

	}

	pub fn get_graph_average(&self) -> f32
	{
		let mut sum: f32 = 0.0;
		let mut i = self.head;
		while i < self.head + self.count {
			let ix = i % CAP;
			sum += self.values[ix as uint];
			i = i+1;
		}
		sum / self.count as f32
	}
}



//#define GPU_QUERY_COUNT 5
//struct GPUtimer {
//	int supported;
//	int cur, ret;
//	unsigned int queries[GPU_QUERY_COUNT];
//};

//void initGPUTimer(struct GPUtimer* timer);
//pub fn initGPUTimer(timer: *mut Struct_GPUtimer);

//void startGPUTimer(struct GPUtimer* timer);
//pub fn startGPUTimer(timer: *mut Struct_GPUtimer);

//int stopGPUTimer(struct GPUtimer* timer, float* times, int maxTimes);
//pub fn stopGPUTimer(timer: *mut Struct_GPUtimer,
//                    times: *mut ::libc::c_float,
//                    maxTimes: ::libc::c_int) -> ::libc::c_int;
