import {combineReducers} from 'redux';
import {counterReducer} from './counter/counterReducer';
import {tracesReducer} from './traces/reducers';
import {generalReducer} from './general/reducers';

export const rootReducer = combineReducers({
  counter: counterReducer,
  traces: tracesReducer,
  general: generalReducer,
});

export type RootState = ReturnType<typeof rootReducer>;
