export type Teleport = {
  name: string;
  directories?: string[];
  to?: string;
};

export type Storage = {
  name: string;
  directory: string;
  constraint?: string;
};

export type Directory = string;

export type TeleportResponse = {
  index: string;
  name: string;
  directories: string[];
  to: string;
};

export type StorageResponse = {
  index: string;
  name: string;
  directory: string;
  constraint: string;
};

export type MappedField = {
  field: string;
  value: string | number;
};

export type UpdateArgs = {
  filename: string;
  target: MappedField;
};

export type TeleportUpdateArgs = UpdateArgs;
export type StorageUpdateArgs = UpdateArgs;
