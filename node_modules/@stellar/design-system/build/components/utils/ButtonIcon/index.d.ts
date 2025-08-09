import React from "react";
import "./styles.scss";
declare enum ButtonIconPosition {
    left = "left",
    right = "right"
}
interface ButtonIconComponent {
    position: typeof ButtonIconPosition;
}
interface ButtonIconProps {
    position: ButtonIconPosition;
    children: React.ReactNode;
}
export declare const ButtonIcon: React.FC<ButtonIconProps> & ButtonIconComponent;
export {};
//# sourceMappingURL=index.d.ts.map