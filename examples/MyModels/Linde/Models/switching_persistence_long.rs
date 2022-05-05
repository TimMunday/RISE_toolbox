% We declare the parameters of the markov chain that will control the
% persistence of the shocks. The parameters of that markov chain are themselves
% controlled by the const markov chain since they are constant.
parameters per_tp_1_2, per_tp_2_1

parameters(per,2) wf,

parameterization
	wf(per,1)          ,	  0.6   ,    0.5000,    0.900,  gamma_pdf(.90);
	wf(per,2)          ,    0.4   ,    0.1,    0.500,  gamma_pdf(.90);
	% transition probabilities
	per_tp_1_2           ,   0.05  ,    0.100,    0.2500,  beta_pdf(.90);
	per_tp_2_1           ,   0.05  ,    0.100,    0.2500,  beta_pdf(.90);

% for identification purposes we need to impose the regime in which a particular
% parameter will assume the greatest value. We choose to identify the second
% regime as the regime with the highest volatility

parameter_restrictions
	wf(per,1)>wf(per,2);
