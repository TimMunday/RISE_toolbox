%% RISE Tutorial by Dr. Tao Zha
%% housekeeping
clear
close all
clc
%% Instructions
% - Please run this file block by block and make sure you read the
% comments in each block to understand what it does. If there is anything
% you do not understand, ask questions to the instructor or to your
% neighbor
% - to run a particular block, click on the block and then on your keyboard
% press CTRL+Enter
%% add the paths to RISE, the data and the models
setpaths=true;
if setpaths
    addpath C:\Users\tmund\Documents\LearningTaylor\Code\RISE\FOMC\RISE_toolbox\examples\MyModels\TaoZha\Models % folder with the models
    addpath C:\Users\tmund\Documents\LearningTaylor\Code\RISE\FOMC\RISE_toolbox\examples\MyModels\TaoZha\Data % folder containing the data
end
%% Bring in some data and transform them into RISE's time series format (ts)

tmp=load('Data/savetest2.mat');  %qdatae
dataList={
    'X','output gap  y_t (log GDP_t - log GDPPotential_t)'
    'PAI','PCE core inflation pi_t (log P_t - log P_{t-1})'
    'R','FFR R_t log(1+ffr/400)  (quarterly rate already)'
    };
mydata=struct();
startdate='1960Q1';
for id=1:size(dataList,1)
    % we just give the start date, RISE automatically understand that we
    % are dealing with quarterly data by the format startdate
    mydata.(dataList{id,1})=ts(startdate,... start date
        tmp.A(:,id+1),... the data
        dataList{id,2});
end

%% plot your data, compute basic statistics and look at both carefully
varlist=fieldnames(mydata);
figure('name','US data')
nvars=numel(varlist);
for id=1:nvars
    subplot(nvars,1,id)
    dd=mydata.(varlist{id});
    plot(dd,'linewidth',2)
    title(mydata.(varlist{id}).varnames)
    fprintf('%s:: mean %0.3f  stdev %0.3f\n',mydata.(varlist{id}).varnames{1},mean(dd),std(dd));
end
[~,tmp]=sup_label(['US data ',mydata.(varlist{id}).start,':', mydata.(varlist{id}).finish],'t');
set(tmp,'fontsize',15)

%% Read the model(s)

model_names = {'volatilityPolicySame'};
% model_names={'volatilityOnly','policyOnly','volatilityPolicySame',...
%     'volatilityPolicyIndependent'};
nmodels=numel(model_names);
estim_models=cell(1,nmodels);

% rather than putting all the models in the same vector as we did earlier,
% we put them in a cell array. If we put them in the same vector and call
% the estimation function, RISE will think that we want to estimate a
% pareto-type of model. But this is not what we want to do and is probably
% beyond the scope of these lectures.

% we loop through the different models using the information in the labels
for imod=1:nmodels 
    % replace "for" by "parfor" if you want to use parallel computation
    estim_models{imod}=rise(model_names{imod},... % name of the file to read
        'saveas',true,... % write the expanded model to disk with the default name
        'data',mydata... % we may assign the data now or later
        );
    % a model with multiple files inserted can be difficult to read. The
    % expanded model could be useful for understanding what RISE does and
    % for debugging purposes. The expanded model contains all the details
    % of the individual files (without the comments)
end

% to make data in mat file from csv
% table = readtable(something.csv)
% then A = table2array(table)
% then save('savetest.mat', 'A')

%% We estimate the models or filter them directly
%close all clc
% if we have the parallel computing toolbox, we can estimate all models in
% one go
filtration=cell(1,nmodels); % used to be curly brackets here
for imod=1:nmodels 
    % replace "for" by "parfor" if you want to use parallel computation
    disp('*--------------------------------------------------------------*')
    disp(['*---------Estimation of ',model_names{imod},' model-----------*'])
    disp('*--------------------------------------------------------------*')
    [estim_models{imod},filtration{imod}]=estimate(estim_models{imod},'optimizer','fmincon');
end

%% plot the smoothed probabilities
% we plot the low response (coef_2) and the high volatility (vol_2) regimes
mystates={'coef_2','vol_2'};
mylabels={'low monetary policy response regime','High volatility regime'};
for imod=1:nmodels
    mytitle=['smoothed probabilities for ',model_names{imod},' model'];
    thisstates=mystates;
    thislabels=mylabels;
    discard=false(1,numel(thisstates));
    for ii=1:numel(thisstates)
        discard(ii)=~ismember(thisstates{ii},estim_models{imod}.markov_chains.state_names);
    end
    thisstates=thisstates(~discard);
    thislabels=thislabels(~discard);
    nstates=numel(thisstates);
    figure('name',mytitle)
    for istate=1:nstates
        subplot(nstates,1,istate)
        plot(filtration{imod}.smoothed_state_probabilities.(thisstates{istate}),...
            'linewidth',2)
        title([thislabels{istate},'(chain ',thisstates{istate}(1:end-2),' state ',thisstates{istate}(end),')'])
    end
    [junk,tmp]=sup_label(mytitle,'t');
    set(tmp,'fontsize',15)
    orient tall    
end

%% My export cell
% export probs of vol 2 regime 
regime_2_probs_policy = filtration{1}.smoothed_regime_probabilities.regime_2.data;
writematrix(regime_2_probs_policy, 'C:\Users\tmund\Documents\LearningTaylor\Data\HF\regime_2_probs_policy.csv');

% export irfs of inlfation and interest rate shocks to 
simple_irfs = irf(estim_models{1},'irf_periods',20);

writematrix(simple_irfs.ED.R.data,'C:\Users\tmund\Documents\LearningTaylor\Data\HF\irfsEDR.csv');
writematrix(simple_irfs.ED.PAI.data,'C:\Users\tmund\Documents\LearningTaylor\Data\HF\irfsEDPAI.csv');
writematrix(simple_irfs.ED.X.data,'C:\Users\tmund\Documents\LearningTaylor\Data\HF\irfsEDX.csv');

writematrix(simple_irfs.ES.R.data,'C:\Users\tmund\Documents\LearningTaylor\Data\HF\irfsESR.csv');
writematrix(simple_irfs.ES.PAI.data,'C:\Users\tmund\Documents\LearningTaylor\Data\HF\irfsESPAI.csv');
writematrix(simple_irfs.ES.X.data,'C:\Users\tmund\Documents\LearningTaylor\Data\HF\irfsESX.csv');

%regime_2_probs_policyvol = filtration{2}.smoothed_regime_probabilities.regime_2.data;
%writematrix(regime_2_probs_policyvol, 'C:\Users\tmund\Documents\LearningTaylor\Data\HF\regime_2_probs_policyvol.csv');
