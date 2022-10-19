import React from "react"
import {Button} from "react-bootstrap";
import buttonStyles from "../styles/button.module.scss";
import topMenuStyles from "../styles/topmenu.module.scss";
import { useDispatch, useSelector } from "react-redux";
import dsLogo from "../assets/images/dragon_stone.png";


export const TopMenu = () => 
{
    let rank : number = useSelector((state : any) => state.user.info.rank);
    let ds : number = useSelector((state : any) => state.user.info.ds);
    let divText : JSX.Element = <div className={topMenuStyles.rankText}>{rank}</div>;
    if (rank >= 0 && rank <= 9)
        divText = <div className={topMenuStyles.rankText}>&nbsp;{rank}</div>;
    else if (rank >= 100)
        divText = (<div className={topMenuStyles.rankText}>
                        <div className={topMenuStyles.rankTextSmall}>{rank}</div>
                   </div>);
    // TODO: Add functionality to click top menu to bring down contents
    return (
        <div>
            <div className={topMenuStyles.menuBar}>
                <div className={topMenuStyles.stoneDisplay}>
                    <img src={dsLogo}/>
                    500000
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