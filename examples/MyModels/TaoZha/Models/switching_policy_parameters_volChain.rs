parameters(vol,2) gamma_1, "$\gamma _{1}$", gamma_2, "$\gamma _{2}$", gamma_x1, gamma_i1, gamma_i2

parameterization
	gamma_1(vol,1) ,    1  ,     0.5000,    1.5000,  gamma_pdf(.90); %2.19
	gamma_1(vol,2) ,    2  ,     1.5000,    3.0000,  gamma_pdf(.90); %0.77
	gamma_2(vol,1) ,    0.1  ,     0.0500,    3.0000,  gamma_pdf(.90); %0.3
	gamma_2(vol,2) ,    0.2  ,     0.0500,    3.0000,  gamma_pdf(.90); %0.17
	gamma_x1(vol, 1) ,  0, -0.5, 0.5, normal_pdf(0.9);
	gamma_x1(vol, 2) , 0, -0.5, 0.5, normal_pdf(0.9);
	gamma_i1(vol, 1) , 0, -0.5, 0.5, normal_pdf(0.9);
	gamma_i1(vol, 2) , 0, -0.5, 0.5, normal_pdf(0.9);
	gamma_i2(vol, 1) , 0, -0.5, 0.5, normal_pdf(0.9);
	gamma_i2(vol, 2) , 0, -0.5, 0.5, normal_pdf(0.9);
