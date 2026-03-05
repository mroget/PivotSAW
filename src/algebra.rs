use crate::math::ArithmeticOps;

enum MatrixKind { 
	Row, 
	Col,  
}

pub type Vector<T,const D : usize> = [T;D];
pub type Matrix<T,const D1 : usize ,const D2 : usize> = [[T;D2];D1];





pub trait IntoMatrix<T, const D1 : usize, const D2 : usize> {
	fn into(self) -> Matrix<T,D1,D2>;
}

impl<T: Copy, const D1 : usize> IntoMatrix<T,D1,1> for Vector<T,D1> {
	fn into(self) -> Matrix<T,D1,1> {
		self.map(|i| [i])
	}
}


impl<T: Copy, const D1 : usize, const D2: usize> IntoMatrix<T,D1,D2> for Matrix<T,D1,D2> {
	fn into(self) -> Matrix<T,D1,D2> {
		self
	}
}




// Helper to pick the right tag
pub trait IntoVector<T,const D: usize> {
    fn from_mat(self) -> Vector<T,D>;
}

impl<T: Copy, const D : usize> IntoVector<T,D> for Vector<T,D> {
	fn from_mat(self) -> Vector<T,D> {
		self
	}
}

impl<T: ArithmeticOps, const D1 : usize, const D2 : usize, const D3 : usize> IntoVector<T,D3> for Matrix<T,D1,D2> {
	fn from_mat(self) -> Vector<T,D3> {
		if D1*D2 != D3 {
			panic!("{D1} x {D2} does not match {D3}");
		}
		let mut ret = [T::zero(); D3];
		let mut k = 0;
		for i in 0..D1 {
			for j in 0..D2 {
				ret[k] = self[i][j];
				k = k + 1;
			}
		}
		ret
	}
}





pub fn vec_add<T : ArithmeticOps,const D : usize>(a : Vector<T,D>, b : Vector<T,D>) -> Vector<T,D> {
	let mut ret = [T::zero();D];
	for i in 0..D {
		ret[i] = a[i] + b[i];
	}
	ret
}

pub fn vec_sub<T : ArithmeticOps,const D : usize>(a : Vector<T,D>, b : Vector<T,D>) -> Vector<T,D> {
	let mut ret = [T::zero();D];
	for i in 0..D {
		ret[i] = a[i] - b[i];
	}
	ret
}

pub fn vec_square_len<T : ArithmeticOps,const D : usize>(a : Vector<T,D>) -> T {
	let mut ret = T::zero();
	for i in 0..D {
		ret = ret + a[i]*a[i];
	}
	ret
}

pub fn vec_len<T : ArithmeticOps + ,const D : usize>(a : Vector<T,D>) -> f64 where f64: From<T> {
	(<T as Into<f64>>::into(vec_square_len(a))).sqrt()
}

pub fn vec_minus<T : ArithmeticOps,const D : usize>(a : Vector<T,D>) -> Vector<T,D> {
	vec_sub([T::zero();D], a)
}








pub fn dot<T : ArithmeticOps,
const D1 : usize,
const D2 : usize,
const D3 : usize, 
M1 : IntoMatrix<T, D1, D2>,
M2 : IntoMatrix<T, D2, D3>,
>(a : M1, b : M2) -> Matrix<T,D1,D3> {
	let mut ret = [[T::zero();D3];D1];
	let a = a.into();
	let b = b.into();
	for i in 0..D1 {
		for j in 0..D3 {
			for k in 0..D2 {
				ret[i][j] = ret[i][j] + a[i][k] * b[k][j];
			}
		}
	}
	ret
}

pub fn transpose<T : ArithmeticOps,
const D1 : usize,
const D2 : usize,
M : IntoMatrix<T, D1, D2>,
>(a : M) -> Matrix<T,D2,D1> {
	let mut ret = [[T::zero();D1];D2];
	let a = a.into();
	for i in 0..D1 {
		for j in 0..D2 {
			ret[j][i] = a[i][j];
		}
	}
	ret
}

pub fn transform<T : ArithmeticOps,const D : usize>(point : Vector<T,D>, q : Matrix<T,D,D>, origin : Vector<T,D>) -> Vector<T,D> {
	vec_add(
		dot(
			q,
			vec_sub(point,origin)
			).from_mat(),
		origin
	)
}