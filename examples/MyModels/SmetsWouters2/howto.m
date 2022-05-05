%% housekeeping
close all
clear
clc

%% load the data and construct the time series
%dat=load('usmodel_data');
%vnames=fieldnames(dat);
%data_start='1947Q3';
%mydata2 = struct();
%data=[];
%for jj=1:numel(vnames)
%    mydata2.(vnames{jj}) = ts(data_start, dat.(vnames{jj}));
%end


%for jj=1:numel(vnames)
%    data=[data,dat.(vnames{jj})];
%end
%data=ts(data_start,data,vnames);

dat=load('usmodel_data');
vnames=fieldnames(dat);
data=[];
for jj=1:numel(vnames)
    data=[data,dat.(vnames{jj})];
end
data_start='1947q3';
data=ts(data_start,data,vnames);

%% read the model and assign the data
sw=rise('usmodel','data',data,...
    'estim_start_date',obs2date(data_start,71),... % retrieve the date of the 71st observation
    'kf_presample',4,...
    'fix_point_maxiter',5000,... % jack up the number of iterations to increase the probability of solving
    'solve_check_stability',false); % save some time by avoiding the checking of stability all the time

%% estimating the model
[sw1, sw2] =estimate(sw, 'optimizer', 'fmincon'); % <--- sw=sw.estimate;
