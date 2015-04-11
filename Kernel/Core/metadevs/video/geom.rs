// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// Core/metadevs/video/mod.rs
/// Geometry types
use _common::*;

#[derive(Copy,Clone,PartialEq,Default)]
pub struct Pos
{
	pub x: u32,
	pub y: u32,
}
#[derive(Copy,Clone,PartialEq,Default)]
pub struct Dims
{
	pub w: u32,
	pub h: u32,
}
#[derive(Copy,Clone,PartialEq,Default)]
pub struct Rect
{
	pub pos: Pos,
	pub dims: Dims,
}


impl Pos
{
	pub fn new(x: u32, y: u32) -> Pos {
		Pos { x: x, y: y }
	}
}

impl Dims
{
	pub fn new(w: u32, h: u32) -> Dims {
		Dims { w: w, h: h }
	}

	pub fn height(&self) -> u32 { self.h }
	pub fn width(&self) -> u32 { self.w }
}

impl Rect
{
	pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
		Rect {
			pos: Pos { x: x, y: y },
			dims: Dims::new(w,h),
		}
	}
	pub fn new_pd(pos: Pos, dims: Dims) -> Rect {
		Rect { pos: pos, dims: dims }
	}
	
	pub fn within(&self, w: u32, h: u32) -> bool {
		self.x() < w && self.y() < h
		&& self.w() <= w && self.h() <= h
		&& self.x() + self.w() <= w && self.y() + self.h() <= h
	}
	
	pub fn pos(&self) -> Pos { self.pos }
	pub fn dims(&self) -> Dims { self.dims }
	
	pub fn x(&self) -> u32 { self.pos.x }
	pub fn y(&self) -> u32 { self.pos.y }
	pub fn w(&self) -> u32 { self.dims.w }
	pub fn h(&self) -> u32 { self.dims.h }
	
	pub fn top(&self) -> u32 { self.y() }
	pub fn left(&self) -> u32 { self.x() }
	pub fn right(&self) -> u32 { self.x() + self.w() }
	pub fn bottom(&self) -> u32 { self.y() + self.h() }
	
	pub fn tl(&self) -> Pos { self.pos }
	pub fn br(&self) -> Pos { Pos::new( self.x() + self.w(), self.y() + self.h() ) }
	pub fn br_inner(&self) -> Pos { Pos::new( self.x() + self.w() - 1, self.y() + self.h() - 1 ) }
	
	pub fn contains(&self, pt: &Pos) -> bool {
		//log_trace!("Rect::contains - self={:?}, pt={:?}", self, pt);
		(self.left() <= pt.x && pt.x < self.right()) && (self.top() <= pt.y && pt.y < self.bottom())
	}
	pub fn contains_rect(&self, r: &Rect) -> bool {
		//log_trace!("Rect::contains - self={:?}, pt={:?}", self, pt);
		if ! self.contains( &r.tl() ) {
			false
		}
		else if r.w() == 0 || r.h() == 0 {
			true
		}
		else if self.contains( &r.br_inner() ) {
			true
		}
		else {
			false
		}
	}
	
	pub fn intersect(&self, other: &Rect) -> Option<Rect> {
		// Intersection:
		//  MAX(X1) MAX(Y1)  MIN(X2) MIN(Y2)
		let max_x1 = ::core::cmp::max( self.left(), other.left() );
		let max_y1 = ::core::cmp::max( self.top() , other.top() );
		let min_x2 = ::core::cmp::min( self.right() , other.right() );
		let min_y2 = ::core::cmp::min( self.bottom(), other.bottom() );
		
		//log_trace!("Rect::intersect({} with {}) = ({},{}) ({},{})", self, other, max_x1, max_y1, min_x2, min_y2);
		
		if max_x1 < min_x2 && max_y1 < min_y2 {
			Some( Rect {
				pos: Pos { x: max_x1, y: max_y1 },
				dims: Dims::new((min_x2 - max_x1), (min_y2 - max_y1))
				} )
		}
		else {
			None
		}
	}
	/// Returns the loose union of two rects (i.e. the smallest rect that contains both)
	pub fn union(&self, other: &Rect) -> Rect
	{
		let new_tl = Pos {
			x: ::core::cmp::min(self.left(), other.left()), 
			y: ::core::cmp::min(self.top(),  other.top() )
			};
		let new_br = Pos {
			x: ::core::cmp::max(self.right(),  other.right() ), 
			y: ::core::cmp::max(self.bottom(), other.bottom())
			};
		Rect {
			pos: new_tl,
			dims: Dims::new( new_br.x - new_tl.x, new_br.y - new_tl.y ),
		}
	}
	
	/// Iterate over intersections of two slices of `Rect`
	pub fn list_intersect<'a>(list1: &'a [Rect], list2: &'a [Rect]) -> RectListIntersect<'a> {
		RectListIntersect {
			list1: list1,
			list2: list2,
			idx1: 0,
			idx2: 0,
		}
	}
}
pub struct RectListIntersect<'a>
{
	list1: &'a [Rect],
	list2: &'a [Rect],
	idx1: usize,
	idx2: usize,
}
impl<'a> Iterator for RectListIntersect<'a>
{
	type Item = Rect;
	fn next(&mut self) -> Option<Rect>
	{
		// Iterate list1, iterate list2
		while self.idx1 < self.list1.len()
		{
			if self.idx2 == self.list2.len() {
				self.idx1 += 1;
				self.idx2 = self.idx1;
				if self.idx2 >= self.list2.len() {
					return None;
				}
			}
			else {
				let rv = self.list1[self.idx1].intersect( &self.list2[self.idx2] );
				self.idx2 += 1;
				if rv.is_some() {
					return rv;
				}
			}
		}
		None
	}
}

impl_fmt! {
	Debug(self, f) for Pos { write!(f, "({},{})", self.x, self.y) }
	Debug(self, f) for Dims { write!(f, "{}x{}", self.w, self.h) }
	Debug(self, f) for Rect { write!(f, "({},{} + {}x{})", self.x(), self.y(), self.w(), self.h()) }
	Display(self, f) for Rect { write!(f, "({},{} + {}x{})", self.x(), self.y(), self.w(), self.h()) }
}
