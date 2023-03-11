const Header = (props) => {
  return (
    <div class="grid grid-cols-6 h-20 bg-black border-b border-neutral-800 items-center">
      <div class="h-full pl-4 pt-3">
        <svg class="fill-slate-300" id="logo" fill="#ffffff" height="40px" width="40px" version="1.1"
          xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewbox="0 0 294 294"
          xml:space="preserve" style="--darkreader-inline-fill: #ffffff;" data-darkreader-inline-fill="">
          <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
          <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
          <g id="SVGRepo_iconCarrier">
            <path
              d="M197.333,134.25c-32.257,0-58.5,26.243-58.5,58.5s26.243,58.5,58.5,58.5c32.257,0,58.5-26.243,58.5-58.5 S229.59,134.25,197.333,134.25z M197.333,216.75c-13.233,0-24-10.767-24-24s10.767-24,24-24s24,10.767,24,24 S210.566,216.75,197.333,216.75z M279.333,95.75H252V50.49c11-2.671,18.166-12.078,18.166-23.24c0-13.233-10.683-24-23.917-24 s-24.125,10.767-24.125,24c0,10.052,6.875,18.674,14.875,22.243V95.75H15.333C7.049,95.75,0,102.466,0,110.75v165 c0,8.284,7.049,15,15.333,15h264c8.284,0,14.667-6.716,14.667-15v-165C294,102.466,287.618,95.75,279.333,95.75z M81.333,250.75h-49 c-4.143,0-7.5-3.357-7.5-7.5s3.357-7.5,7.5-7.5h49c4.143,0,7.5,3.357,7.5,7.5S85.476,250.75,81.333,250.75z M81.333,200.75h-49 c-4.143,0-7.5-3.357-7.5-7.5s3.357-7.5,7.5-7.5h49c4.143,0,7.5,3.357,7.5,7.5S85.476,200.75,81.333,200.75z M81.333,150.75h-49 c-4.143,0-7.5-3.357-7.5-7.5s3.357-7.5,7.5-7.5h49c4.143,0,7.5,3.357,7.5,7.5S85.476,150.75,81.333,150.75z M197.333,266.25 c-40.528,0-73.5-32.972-73.5-73.5s32.972-73.5,73.5-73.5c40.528,0,73.5,32.972,73.5,73.5S237.861,266.25,197.333,266.25z">
            </path>
          </g>
        </svg>
      </div>
      <div class="col-span-4">{props.children}</div>
      <div></div>
    </div>
  )
}

export default Header


