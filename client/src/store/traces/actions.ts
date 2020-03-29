import {
  Location,
  ADD_LOCATION,
  DELETE_LOCATIONS,
  LocationActionTypes,
} from './types';

export function addLocation(location: Location): LocationActionTypes {
  return {
    type: ADD_LOCATION,
    payload: location,
  };
}

export function deleteLocations(): LocationActionTypes {
  return {
    type: DELETE_LOCATIONS,
  };
}
