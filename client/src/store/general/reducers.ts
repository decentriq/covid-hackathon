import {
  CurrentStatus,
  CHANGE_STATUS,
  CHANGE_ENCLAVE_IDENT,
  GeneralActionTypes,
  GeneralStore,
  CHANGE_ILLNESS,
} from './types';

const initialState: GeneralStore = {
  enclave_identity: 'unknown',
  current_status: CurrentStatus.Recording,
  illness: null,
};

export function generalReducer(
  state = initialState,
  action: GeneralActionTypes,
): GeneralStore {
  switch (action.type) {
    case CHANGE_STATUS:
      return {
        ...state,
        current_status: action.new_status,
      };
    case CHANGE_ENCLAVE_IDENT:
      return {
        ...state,
        enclave_identity: action.new_ident,
      };
    case CHANGE_ILLNESS:
      return {
        ...state,
        illness: action.illness,
      };
    default:
      return state;
  }
}
