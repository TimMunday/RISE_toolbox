%-------------------------------------------------------------%
%                   Declarations:
%     names are separated by a comma, a space or both
%     "..." are used to describe the preceding variable
%-------------------------------------------------------------%

%Endogenous variables
endogenous	 X, "Output gap", PAI, "Inflation", R, "Fed Funds rate",
ZS, "Supply shock process", ZD "Demand shock process"

%Exogenous variables
exogenous ES,  "Supply shock", ED, "Demand shock", ER, "Monetary policy shock"

%parameters
parameters tau, "$\tau $", beta_trans, "$100\left( \frac{1}{\beta }-1\right) $", 
kappa, "$\kappa $", rhor, "$\rho _{r}$",
rhod, "$\rho _{d}$", sigd, "$\sigma_{d}", sigr, "\sigma_{r}", sigs, "\sigma_{s}"
% N.B: we have removed the transition probabilities from the list of
% parameters since in some cases they will not matter
% N.B: we replace beta by beta_trans, just to make optimization easier
% the relationship between the two is given in the model

% observable variables
varobs R, X, PAI


model
	% auxiliary parameters
	# beta=1/(1+beta_trans/100);
	
   % Main equations
   % N.B: time is denoted by () as in dynare or by {}. Below, we use the {} notation
   X   = X{+1}-tau*(R-PAI{+1})+ZD;
   
   PAI = beta*PAI{+1}+kappa*X+ZS;
   
   R   = rhor*R{-1}+(1-rhor)*(gamma_1*PAI+gamma_2*X)+sigr*ER;

   % Shock processes
   ZD = rhod*ZD{-1}+sigd*ED;
   
   ZS = rhos*ZS{-1}+sigs*ES;

% Planners objective
planner_objective -.5*(PAI^2 + 0.3*X^2); % but how does RISE know to only change the gammas for this?

% the non-policy parameters never switch, they will be controlled by the const markov chain
parameterization
	tau          ,    0.5376,     0.1000,    0.5000,  gamma_pdf(.90);
	kappa        ,    0.5800,     0.0500,    1.0000,  gamma_pdf(.90);
	beta_trans   ,    0.1000,     0.2000,    0.4000,  beta_pdf(.90);
   sigd         ,    0.18   ,    0.0005,    1.0000,  weibull_pdf(.90);
   sigs         ,    0.3712 ,    0.0005,    1.0000,  weibull_pdf(.90);
   sigr         ,    0.18   ,    0.0005,    1.0000,  weibull_pdf(.90);
	rhod         ,    0.83  ,     0.5000,    0.9000,  beta_pdf(.90);
	rhor         ,    0.60  ,     0.5000,    0.9000,  beta_pdf(.90);
