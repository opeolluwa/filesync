import { DesktopApp } from "./desktop";
import { MobileApp } from "./mobile";


const IS_MOBILE = true;

export default function FileSync(){
  if(IS_MOBILE){
    return <MobileApp/>
  }
  else {
    return <DesktopApp/>
  }
}