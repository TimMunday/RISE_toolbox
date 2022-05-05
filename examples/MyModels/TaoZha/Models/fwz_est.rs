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
parameters tau, "$\tau $", beta,
kappa, "$\kappa $",
rhod, "$\rho _{d}$" rhos, "$\rho _{s}$",

% N.B: we have removed the transition probabilities from the list of
% parameters since in some cases they will not matter
% N.B: we replace beta by beta_trans, just to make optimization easier
% the relationship between the two is given in the model

% observable variables
varobs R, X, PAI


model
	
   % Main equations
   % N.B: time is denoted by () as in dynare or by {}. Below, we use the {} notation
   X   = X{+1}-tau*(R-PAI{+1})+ZD;
   
   PAI = beta*PAI{+1}+kappa*X+ZS;
   
   R   = (gamma_1*PAI+gamma_2*X)+ gamma_x1*X{-1} + gamma_i1*R{-1} + gamma_i2*R{-2} + sigr*ER;

   % Shock processes
   ZD = rhod*ZD{-1}+sigd*ED;
   
   ZS = rhos*ZS{-1}+sigs*ES;


% the non-policy parameters never switch, they will be controlled by the const markov chain
parameterization
	tau          ,    0.5; %     0.1000,    0.5000,  gamma_pdf(.90);
	kappa        ,    0.4;%     0.0500,    1.0000,  gamma_pdf(.90);
	beta   ,    0.99;
	% for simplicity, we also assume that the persistence of the shocks is constant
	% change this if you don't like the assumption
	rhod         ,    0.7;
	rhos         ,    0.7;
	
