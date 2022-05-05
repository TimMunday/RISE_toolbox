endogenous ewma epinfma  zcapf rkf kf pkf cf invef yf labf wf rrf mc zcap rk k pk c inve y lab pinf w r a
b g qs  ms  spinf sw kpf kp

endogenous labobs robs pinfobs dy dc dinve dw
 
exogenous ea eb eg  eqs  em  epinf ew   
 
parameters curvw cgy curvp constelab constepinf constebeta cmaw cmap calfa 
czcap csadjcost ctou csigma chabb cfc 
cindw cprobw cindp cprobp csigl clandaw  crdy crr 
crhoa crhoas crhob crhog crhols crhoqs crhoms crhow  
ctrend cg, sig_a, sig_b,sig_w,sig_pinf,sig_m,sig_qs,sig_g

% changing parameters
parameters(per, 2) crpi cry crhopinf
parameters per_tp_1_2, per_tp_2_1


model 

		# cpie=1+constepinf/100;
		# cgamma=1+ctrend/100 ;
		# cbeta=1/(1+constebeta/100);
		# clandap=cfc;
		# cbetabar=cbeta*cgamma^(-csigma);
		# cr=cpie/(cbeta*cgamma^(-csigma));
		# crk=(cbeta^(-1))*(cgamma^csigma) - (1-ctou);
		# cw = (calfa^calfa*(1-calfa)^(1-calfa)/(clandap*crk^calfa))^(1/(1-calfa));
		# cikbar=(1-(1-ctou)/cgamma);
		# cik=(1-(1-ctou)/cgamma)*cgamma;
		# clk=((1-calfa)/calfa)*(crk/cw);
		# cky=cfc*(clk)^(calfa-1);
		# ciy=cik*cky;
		# ccy=1-cg-cik*cky;
		# crkky=crk*cky;
		# cwhlc=(1/clandaw)*(1-calfa)/calfa*crk*cky/ccy;
		# cwly=1-crk*cky;
		# conster=(cr-1)*100;

% flexible economy

	      0*(1-calfa)*a + 1*a =  calfa*rkf+(1-calfa)*(wf)  ;
	      zcapf =  (1/(czcap/(1-czcap)))* rkf  ;
	      rkf =  (wf)+labf-kf ;
	      kf =  kpf(-1)+zcapf ; % A.5
	      invef = (1/(1+cbetabar*cgamma))* (  invef(-1) + cbetabar*cgamma*invef(1)+(1/(cgamma^2*csadjcost))*pkf ) +qs ;
          pkf = -rrf-0*b+(1/((1-chabb/cgamma)/(csigma*(1+chabb/cgamma))))*b +(crk/(crk+(1-ctou)))*rkf(1) +  ((1-ctou)/(crk+(1-ctou)))*pkf(1) ;
	      cf = (chabb/cgamma)/(1+chabb/cgamma)*cf(-1) + (1/(1+chabb/cgamma))*cf(+1) +((csigma-1)*cwhlc/(csigma*(1+chabb/cgamma)))*(labf-labf(+1)) - (1-chabb/cgamma)/(csigma*(1+chabb/cgamma))*(rrf+0*b) + b ;
	      yf = ccy*cf+ciy*invef+g  +  crkky*zcapf ;
	      yf = cfc*( calfa*kf+(1-calfa)*labf +a ); 
	      wf = csigl*labf 	+(1/(1-chabb/cgamma))*cf - (chabb/cgamma)/(1-chabb/cgamma)*cf(-1) ;
	      kpf =  (1-cikbar)*kpf(-1)+(cikbar)*invef + (cikbar)*(cgamma^2*csadjcost)*qs ;

% sticky price - wage economy

	      mc =  calfa*rk+(1-calfa)*(w) - 1*a - 0*(1-calfa)*a ;
	      zcap =  (1/(czcap/(1-czcap)))* rk ; % A.4
	      rk =  w+lab-k ;
	      k =  kp(-1)+zcap ; % A.5
	      inve = (1/(1+cbetabar*cgamma))* (  inve(-1) + cbetabar*cgamma*inve(1)+(1/(cgamma^2*csadjcost))*pk ) +qs ;
          pk = -r+pinf(1)-0*b +(1/((1-chabb/cgamma)/(csigma*(1+chabb/cgamma))))*b + (crk/(crk+(1-ctou)))*rk(1) +  ((1-ctou)/(crk+(1-ctou)))*pk(1) ;
	      c = (chabb/cgamma)/(1+chabb/cgamma)*c(-1) + (1/(1+chabb/cgamma))*c(+1) +((csigma-1)*cwhlc/(csigma*(1+chabb/cgamma)))*(lab-lab(+1)) - (1-chabb/cgamma)/(csigma*(1+chabb/cgamma))*(r-pinf(+1) + 0*b) +b ;
	      y = ccy*c+ciy*inve+g  +  1*crkky*zcap ;
	      y = cfc*( calfa*k+(1-calfa)*lab +a ); % A.8
	      pinf =  (1/(1+cbetabar*cgamma*cindp)) * ( cbetabar*cgamma*pinf(1) +cindp*pinf(-1) 
               +((1-cprobp)*(1-cbetabar*cgamma*cprobp)/cprobp)/((cfc-1)*curvp+1)*(mc)  )  + spinf ; 
	      w =  (1/(1+cbetabar*cgamma))*w(-1)
               +(cbetabar*cgamma/(1+cbetabar*cgamma))*w(1)
               +(cindw/(1+cbetabar*cgamma))*pinf(-1)
               -(1+cbetabar*cgamma*cindw)/(1+cbetabar*cgamma)*pinf
               +(cbetabar*cgamma)/(1+cbetabar*cgamma)*pinf(1)
               +(1-cprobw)*(1-cbetabar*cgamma*cprobw)/((1+cbetabar*cgamma)*cprobw)*(1/((clandaw-1)*curvw+1))*
               (csigl*lab + (1/(1-chabb/cgamma))*c - ((chabb/cgamma)/(1-chabb/cgamma))*c(-1) -w) 
               + 1*sw ; % A.12
	      r =  crpi*(1-crr)*pinf +cry*(1-crr)*(y-yf)+crdy*(y-yf-y(-1)+yf(-1))+crr*r(-1)+ms  ;

% new variable created to re-introduce the monetary policy shock in optimal policy
% and avoid stochastic singularity under the estimation of the optimal policy model
%		  dr=r-r(-1);
%		  pinf_target=pinf+ms;
		  
		  a = crhoa*a(-1)  + sig_a*ea;
	      b = crhob*b(-1) + sig_b*eb;
	      g = crhog*(g(-1)) + sig_g*eg + cgy*sig_a*ea;
	      qs = crhoqs*qs(-1) + sig_qs*eqs;
	      ms = crhoms*ms(-1) + sig_m*em;
	      spinf = crhopinf*spinf(-1) + epinfma - cmap*epinfma(-1);
	          epinfma=sig_pinf*epinf;
	      sw = crhow*sw(-1) + ewma - cmaw*ewma(-1) ;
	          ewma=sig_w*ew; 
	      kp =  (1-cikbar)*kp(-1)+cikbar*inve + cikbar*cgamma^2*csadjcost*qs ;

% measurment equations

		dy=y-y(-1)+ctrend;
		dc=c-c(-1)+ctrend;
		dinve=inve-inve(-1)+ctrend;
		dw=w-w(-1)+ctrend;
		pinfobs = 1*(pinf) + constepinf;
		robs =    1*(r) + conster;
		labobs = lab + constelab;

observables dy dc dinve dw pinfobs robs labobs

% planner_objective{discount = 0.99,commitment=gamma_prob} -.5*(1*pinf_target^2+wy*y^2+wr*dr^2);	

steady_state_model
	dy=ctrend;
	dc=ctrend;
	dinve=ctrend;
	dw=ctrend;
	pinfobs=constepinf;
	robs = conster;
	labobs = constelab;

parameterization
	% fixed parameters
	crhoas,1; 	 % this parameter does not enter the model
	crhols,    0.9928;  % this parameter does not enter the model
	ctou,.025; % depreciation rate
	clandaw,1.5; % gross wage markup
	cg,0.18; % government G/Y ratio in ss
	curvp,10; % kimball curvature GM
	curvw,10; % kimball curvature LM
	
	crpi(per,1)          ,	  1.6,1,2,normal_pdf(.9); % 1.488,1,2,normal_pdf(.9) % taylor rule inflation weight
	crpi(per,2)          ,    0.488,0,1,normal_pdf(.9); %taylor rule inflation weight
	cry(per,1)			 ,    0.05,0.025,0.225,normal_pdf(.9); % 0.0593,0.025,0.225,normal_pdf(.9) %taylor rule output weight
	cry(per,2)		     ,    0.15,0.025,0.225,normal_pdf(.9); %taylor rule output weight
	crhopinf(per,1)		,	0.5,0.2,0.6,beta_pdf(.9); % 0.8,0.3,0.7,beta_pdf(.9) % price markup shock persistence
	crhopinf(per,2)  	,	0.92,0.5,0.9,beta_pdf(.9);% price markup shock persistence
	% transition probabilities
	per_tp_1_2           ,   0.05  ,    0.100,    0.2500,  beta_pdf(.90); % transition prob
	per_tp_2_1           ,   0.05  ,    0.100,    0.2500,  beta_pdf(.90); % transition prob

    % initialisation from challenges for cental banks macro models, no zlb model
	% estimated parameters initialisation
	sig_a,   0.46,0.01,2,inv_gamma_pdf(0.9); % stationary tech shock var
	sig_b,   0.19,0.01,2,inv_gamma_pdf(0.9); % risk premium shock var
	sig_g,   0.6090,0.01,2,inv_gamma_pdf(0.9); % gov cons shock var
	sig_qs,  0.36,0.01,2,inv_gamma_pdf(0.9); % investment specific tech shock var
	sig_m,   0.23,0.01,2,inv_gamma_pdf(0.9); % mon pol shock var
	sig_pinf,0.12,0.01,2,inv_gamma_pdf(0.9); % price markup shock var
	sig_w,   0.37,0.01,2,inv_gamma_pdf(0.9); % wage markup shock var

	calfa,    0.19, .2,.4,normal_pdf(.9); % capital production share
	csigma,   1.49,.75,2.25,normal_pdf(0.9); % investment subs. elasticity of consumption
	cfc,      1.6, 1, 1.5,normal_pdf(.9); % gross price markup
	cgy,      0.51,.1,1.5,normal_pdf(.9); % response of g to epsilon
	
	csadjcost, 4.58,1,7,normal_pdf(0.9); % investment adjustment cost
	chabb,     0.62,0.3,0.7,beta_pdf(.9); % habit formation    
	cprobw,    0.83,0.3,0.7,beta_pdf(.9); % calvo prob wages
	csigl,     1.81,.5,3.5,normal_pdf(.9); % labour supply elasticity
	cprobp,    0.75,0.3,0.7,beta_pdf(.9); % calvo prob prices
	cindw,     0.69,0.3,0.7,beta_pdf(.9); % indexation of wages
	cindp,     0.22,0.3,0.7,beta_pdf(.9); % indexation of prices
	czcap,     0.8,0.3,0.7,beta_pdf(.9); % capital utilization cost

	%crpi,      1.488,1,2,normal_pdf(.9); % taylor rule inflation weight
	crr,       0.8,0.3,0.7,beta_pdf(.9); % taylor rule interest rate smoothing weight
	%cry,       0.0593,0.025,0.225,normal_pdf(.9); % taylor rule output weight
	crdy,      0.24,0.025,0.225,normal_pdf(.9); % taylor rule weight on change in output
	
	crhoa,     0.96,0.2,0.8,beta_pdf(.9); % stationary tech shock persistence
	crhob,     0.4,0.3,0.7,beta_pdf(.9); % risk premium shock persistence
	crhog,     0.49,0.2,0.8,beta_pdf(.9); % gov cons shock persistence
	crhoqs,    0.84,0.3,0.7,beta_pdf(.9); % investment specific tech shock persistence
	crhoms,    0.21,0.3,0.7,beta_pdf(.9); % mon pol shock persistence
	%crhopinf,  0.8,0.3,0.7,beta_pdf(.9); % price markup shock persistence
	crhow,     0.97,0.3,0.7,beta_pdf(.9); % wage markup shock persistence
	cmap ,     0.8,0.3,0.7,beta_pdf(.9); % MA price markup shock
	cmaw  ,    0.96,0.3,0.7,beta_pdf(.9); % MA wage markup shock
	
    % derived from steady state
	constebeta, 0.1,.05,.8,gamma_pdf(.9); % discount factor
	ctrend,     0.41,.2,.6,normal_pdf(.9); % quarterly growth in steady state
	constepinf, .7,.425,.825,gamma_pdf(.9); % ss inflation
	constelab,  -0.4,-4,4,normal_pdf(.9); % hours worked in ss

parameter_restrictions
	crhopinf(per,2)>crhopinf(per,1);

