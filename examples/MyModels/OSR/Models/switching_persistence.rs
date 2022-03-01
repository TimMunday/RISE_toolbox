% We declare the parameters of the markov chain that will control the
% persistence of the shocks. The parameters of that markov chain are themselves
% controlled by the const markov chain since they are constant.
parameters per_tp_1_2, per_tp_2_1

parameters(per,2) rhos, "$\rho_{s}$"

parameterization
	rhos(per,1)          ,	  0.98   ,    0.6000,    0.9500,  beta_pdf(.90);
	rhos(per,2)          ,    0.70   ,    0.4000,    0.7500,  beta_pdf(.90);
	% transition probabilities
	per_tp_1_2           ,   0.0528  ,    0.100,    0.2500,  beta_pdf(.90);
	per_tp_2_1           ,   0.0528  ,    0.100,    0.2500,  beta_pdf(.90);

% for identification purposes we need to impose the regime in which a particular
% parameter will assume the greatest value. We choose to identify the second
% regime as the regime with the highest volatility

parameter_restrictions
	rhos(per,1)>rhos(per,2);
