import {Illness} from '../../services/api';

export type GeneralStore = {
  enclave_identity: any;
  current_status: CurrentStatus;
  illness: Illness | null;
};

export enum CurrentStatus {
  Recording,
  Exposed,
  Infected,
}

export const CHANGE_STATUS = 'CHANGE_STATUS';
export const CHANGE_ENCLAVE_IDENT = 'CHANGE_ENCLAVE_IDENT';
export const CHANGE_ILLNESS = 'CHANGE_ILLNESS';

interface ChangeStatus {
  type: typeof CHANGE_STATUS;
  new_status: CurrentStatus;
}

interface ChangeReport {
  type: typeof CHANGE_ENCLAVE_IDENT;
  new_ident: any;
}

interface ChangeIllness {
  type: typeof CHANGE_ILLNESS;
  illness: Illness;
}

export type GeneralActionTypes = ChangeStatus | ChangeReport | ChangeIllness;
