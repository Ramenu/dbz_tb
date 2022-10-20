import React from "react"
import {Button} from "react-bootstrap";
import buttonStyles from "../styles/button.module.scss";
import topMenuStyles from "../styles/topmenu.module.scss";
import { useDispatch, useSelector } from "react-redux";
import dsLogo from "../assets/images/dragon_stone.png";
import zeniLogo from "../assets/images/zeni.png";
import { User } from "../tsdependencies/User";


export const TopMenu = () => 
{
    const user : User = useSelector((state : any) => state.user.info);
    let divText : JSX.Element = <div className={topMenuStyles.rankText}>{user.rank}</div>;
    if (user.rank >= 0 && user.rank <= 9)
        divText = (<div className={topMenuStyles.rankText}>
                    <div className={topMenuStyles.alignToLeft}>&nbsp;{user.rank}</div>
                  </div>);
    else if (user.rank >= 100)
        divText = (<div className={topMenuStyles.rankText}>
                        <div className={topMenuStyles.small}>{user.rank}</div>
                   </div>);
    // TODO: Add functionality to click top menu to bring down contents
    return (
        <div>
            <div className={topMenuStyles.menuBar}>
                <div className={topMenuStyles.zeniDisplay}>
                    <img src={zeniLogo} alt="Zeni"/>
                    {user.zeni}
                </div>
                <div className={topMenuStyles.stoneDisplay}>
                    <img src={dsLogo} alt="Dragon Stones"/>
                    {user.ds}
                </div>
            </div>
            <div className={topMenuStyles.rankCircle}>
                {divText}
            </div>
        </div>
    );

}

export const SummonMenu = () => {
    return (
        <Button className={buttonStyles.tbbutton}>CLICK HERE!</Button>
    );
}