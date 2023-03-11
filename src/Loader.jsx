const Loader = (props) => {

  return (<div class={`h-[1px] ${props.pending ? 'animate-pulse bg-slate-300' : ''}`}></div>)
}

export default Loader
