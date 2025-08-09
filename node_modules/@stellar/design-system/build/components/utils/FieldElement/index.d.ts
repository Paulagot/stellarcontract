import React from "react";
import "./styles.scss";
declare enum FieldElementPosition {
    left = "left",
    right = "right"
}
interface FieldElementComponent {
    position: typeof FieldElementPosition;
}
interface FieldElementProps {
    position: FieldElementPosition;
    children: React.ReactNode;
}
export declare const FieldElement: React.FC<FieldElementProps> & FieldElementComponent;
export {};
//# sourceMappingURL=index.d.ts.map