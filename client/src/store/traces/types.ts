export type Location = any;

export type LocationStore = {
  locations: Location[];
};

export const ADD_LOCATION = 'ADD_LOCATION';
export const DELETE_LOCATIONS = 'DELETE_LOCATIONS';

interface AddLocationAction {
  type: typeof ADD_LOCATION;
  payload: Location;
}

interface DeleteLocationAction {
  type: typeof DELETE_LOCATIONS;
}

export type LocationActionTypes = AddLocationAction | DeleteLocationAction;
