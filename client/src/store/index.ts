import {combineReducers} from 'redux';
import {counterReducer} from './counter/counterReducer';
import {tracesReducer} from './traces/reducers';

export const rootReducer = combineReducers({
  counter: counterReducer,
  traces: tracesReducer,
});

export type RootState = ReturnType<typeof rootReducer>;
