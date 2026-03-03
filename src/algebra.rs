use crate::math::ArithmeticOps;

pub type Vector<T,const D : usize> = [T;D];

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
		ret = ret + a[i];
	}
	ret
}

pub fn vec_len<T : ArithmeticOps + ,const D : usize>(a : Vector<T,D>) -> f64 where f64: From<T> {
	(<T as Into<f64>>::into(vec_square_len(a))).sqrt()
}

//pub type Matrix<T,const D : usize> = [T;D];
