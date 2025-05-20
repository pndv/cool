class A2I {
    i2a_aux(i : Int) : String {
		i2a_aux(next).concat(i2c(i - next * 10))
    };
};
