import React from "react"
import buttonStyles from "../styles/button.module.scss";
import topMenuStyles from "../styles/topmenu.module.scss";
import bannerStyles from "../styles/banner.module.scss";
import { useDispatch, useSelector } from "react-redux";
import { getExpPercentage, User } from "../tsdependencies/User";
import { Banner, ALL_BANNERS } from "../tsdependencies/Banner";
import { DS_ICON, ZENI_ICON } from "../tsdependencies/ImgSources";
import { multiSummon, singleSummon } from "../redux/slices/UserSlice";


export const TopMenu = () => 
{
    const user : User = useSelector((state : any) => state.user);
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
                    <img src={ZENI_ICON} alt="Zeni"/>
                    {user.zeni}
                </div>
                <div className={topMenuStyles.stoneDisplay}>
                    <img src={DS_ICON} alt="Dragon Stones"/>
                    {user.ds}
                </div>
                <progress className={topMenuStyles.expBar} value={getExpPercentage(user)} max="1.0"/>
            </div>
            <div className={topMenuStyles.rankCircle}>
                {divText}
            </div>
        </div>
    );

}

export const BannerDisplay : React.FC<{banner : Banner}> = ({banner}) => {
    const dispatch = useDispatch();
    return (
        <div className={bannerStyles.banner}>
            <img className={bannerStyles.bannerImg} src={banner.image} alt="Banner"/>
            <div style={{
                position: "relative",
                width: "200px",
                height: "50px",
                paddingTop: "5px",
                left: "40%"
            }}>
                <button className={buttonStyles.tbbutton} onClick={() => dispatch(singleSummon(banner))}>SINGLE SUMMON (5x)</button>
            </div>
            <div style={{
                position: "relative",
                width: "200px",
                height: "50px",
                top: "-52px",
                left: "54%"
            }}>
                <button className={buttonStyles.tbbutton} onClick={() => dispatch(multiSummon(banner))}>MULTI SUMMON (50x)</button>
            </div>
        </div>
    );
}

export const SummonMenu = () : JSX.Element => {
    return (
        <div>
            <BannerDisplay banner={ALL_BANNERS[0]}/>
        </div>
    );
}