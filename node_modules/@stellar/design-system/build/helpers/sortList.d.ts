import { SortOrder } from "../types/types";
type GenericObject = {
    [key: string]: any;
};
export declare const sortList: <T extends GenericObject>(data: T[], sortByKey: string, sortOrder?: SortOrder) => T[];
export {};
//# sourceMappingURL=sortList.d.ts.map