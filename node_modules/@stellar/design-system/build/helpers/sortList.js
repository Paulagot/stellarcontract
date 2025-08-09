import { __spreadArray } from "tslib";
import BigNumber from "bignumber.js";
import { get } from "lodash";
import { SortOrder } from "../types/types";
export var sortList = function (data, sortByKey, sortOrder) {
    if (!data.length) {
        return data;
    }
    var isDescOrder = sortOrder === SortOrder.DESC;
    return __spreadArray([], data, true).sort(function (a, b) {
        var valA = get(a, sortByKey);
        var valB = get(b, sortByKey);
        var numA = new BigNumber(valA);
        var numB = new BigNumber(valB);
        if (numA.isNaN() || numB.isNaN()) {
            var datetimeA = new Date(valA).getTime();
            var datetimeB = new Date(valB).getTime();
            if (!isNaN(datetimeA) && !isNaN(datetimeB)) {
                return isDescOrder ? datetimeA - datetimeB : datetimeB - datetimeA;
            }
            var stringA = valA.toUpperCase();
            var stringB = valB.toUpperCase();
            if (stringA > stringB) {
                return isDescOrder ? -1 : 1;
            }
            if (stringA < stringB) {
                return isDescOrder ? 1 : -1;
            }
            return 0;
        }
        if (numA.gt(numB)) {
            return isDescOrder ? -1 : 1;
        }
        if (numA.lt(numB)) {
            return isDescOrder ? 1 : -1;
        }
        return 0;
    });
};
//# sourceMappingURL=sortList.js.map