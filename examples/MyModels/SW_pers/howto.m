%% housekeeping
close all
clear
clc
%% load the toolbox
rise_startup()
%% load the data and construct the time series

tmp=load('data_2019.mat');  %qdatae data_nk3eq_8501_1301.mat
dataList={'dc', 'dc'
    'dinve', 'dinve'
    'dy', 'dy'
    'labobs', 'labobs'
    'pinfobs', 'pinfobs'
    'dw', 'dw'
    'robs', 'robs'};
mydata=struct();
startdate='1947q3';
for id=1:size(dataList,1)
    % we just give the start date, RISE automatically understand that we
    % are dealing with quarterly data by the format startdate
    mydata.(dataList{id,1})=ts(startdate,... start date
        tmp.A(:,id),... the data %.qdatae
        dataList{id,2});
end


%dat=load('data_2019'); %usmodel_data
%vnames=fieldnames(dat);
%data=[];
%for jj=1:numel(vnames)
%    data=[data,dat.(vnames{jj})];
%end
%data_start='1947q3';
%data=ts(data_start,data,vnames);


% to make data in mat file from csv
% table = readtable(something.csv)
% then A = table2array(table)
% then save('savetest.mat', 'A')
%% read the model and assign the data
sw=rise('usmodel_sr_switch_update','data',mydata,...
    'estim_start_date',obs2date(startdate,151),... % retrieve the date of the where to stat observation, usually 71
    'kf_presample',4,...
    'fix_point_maxiter',5000,... % jack up the number of iterations to increase the probability of solving
    'solve_check_stability',true); % save some time by avoiding the checking of stability all the time
    
% change max f count or max iter
%% estimating the model
[sw_est, sw_filt]=estimate(sw); % <--- sw=sw.estimate;

%% My regime stuff
%%
regime_2_probs = sw_filt.smoothed_regime_probabilities.regime_2.data;
plot(regime_2_probs)
%% My export cell
% export probs of regime 
writematrix(regime_2_probs, 'C:\Users\tmund\Documents\LearningTaylor\Data\HF\regime_2_probs_pers_sw_85.csv');