import {
  LocationActionTypes,
  LocationStore,
  ADD_LOCATION,
  DELETE_LOCATIONS,
} from './types';

const initialState: LocationStore = {
  locations: [],
};

export function tracesReducer(
  state = initialState,
  action: LocationActionTypes,
): LocationStore {
  switch (action.type) {
    case ADD_LOCATION:
      return {
        locations: [...state.locations, action.payload],
      };
    case DELETE_LOCATIONS:
      return {
        locations: [],
      };
    default:
      return state;
  }
}
