import {Illness} from '../../services/api';

import {
  CurrentStatus,
  CHANGE_STATUS,
  CHANGE_ENCLAVE_IDENT,
  CHANGE_ILLNESS,
  GeneralActionTypes,
} from './types';

export function changeStatus(new_status: CurrentStatus): GeneralActionTypes {
  return {
    type: CHANGE_STATUS,
    new_status: new_status,
  };
}

export function changeEnlcaveIdent(identity: any): GeneralActionTypes {
  return {
    type: CHANGE_ENCLAVE_IDENT,
    new_ident: identity,
  };
}

export function changeIllness(illness: Illness): GeneralActionTypes {
  return {
    type: CHANGE_ILLNESS,
    illness: illness,
  };
}
