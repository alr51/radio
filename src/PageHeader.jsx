export default function PageHeader(props) {
  return (
    <div class="sticky top-0 z-20 py-1 px-4 text-xl uppercase font-bold bg-gradient-to-b from-black to-neutral-900">
      <h1 class="inline text-transparent bg-clip-text bg-gradient-to-r from-gray-200 via-gray-400 to-gray-600">{props.title}</h1>
    </div>)
}
