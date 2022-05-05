%-------------------------------------------------------------%
%                   Declarations:
%     names are separated by a comma, a space or both
%     "..." are used to describe the preceding variable
%-------------------------------------------------------------%

%Endogenous variables
endogenous	 X, "Output gap", PAI, "Inflation", R, "Fed Funds rate"

%Exogenous variables
exogenous ES,  "Supply shock", ED, "Demand shock", ER, "Monetary policy shock"

%parameters
parameters sigd, sigr, sigs,
betaf_trans, betay, betar, gamma1, rho1, rho2
% N.B: we have removed the transition probabilities from the list of
% parameters since in some cases they will not matter
% N.B: we replace beta by beta_trans, just to make optimization easier
% the relationship between the two is given in the model

% observable variables
varobs R, X, PAI


model	
   % Main equations
   % N.B: time is denoted by () as in dynare or by {}. Below, we use the {} notation
   % auxiliary params
   # betaf = 1/(1+betaf_trans/100);

   X   = betaf*X{+1} + (1-betaf)*(betay*X{-1} + (1-betay)*X{-2}) - betar*(R-PAI{+1}) + sigd*ED;
   
   PAI = wf*PAI{+1}+ (1-wf)*PAI{-1} + gamma1*X + sigs*ES;
   
   R   = (1 - rho1 - rho2)*(gammapi*PAI + gammay*X) + rho1*R{-1} + rho2*R{-2} + sigr*ER;


% the non-policy and non-persistence parameters never switch, they will be controlled by the const markov chain
parameterization
	betaf_trans          ,    0.17;%,     0.1000,    0.5000,  beta_pdf(.90); % calib beta value is 0.999 for beta_trans = 0.1 , sensible? % calib from one
	gamma1        ,    0.0027;%,    0.005,    0.400,  gamma_pdf(.90); % no reason for this one? works if calib to 0.2 ish, also works if starting value 0.2 % calib from one
   sigd         ,    0.05   ,    0.05,    1.,  weibull_pdf(.90); %guess
   sigs         ,    0.05 ,    0.05,    1.,  weibull_pdf(.90); %guess
   sigr         ,    0.05   ,    0.05,    1.,  weibull_pdf(.90); %guess
   betar        ,    0.0048; %    0.0005,     0.1,     gamma_pdf(.90); % starting value from SW
   betay        ,    1.100; %     1.0,        1.2,     normal_pdf(.90); % starting value from SW and one
   rho1         ,    0.0; %        0.4,        1.5,    gamma_pdf(.90); % calib by me
   rho2         ,    0.0; %       0.0,        0.3,     normal_pdf(.90); % calib by me


