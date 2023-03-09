import { HiOutlineSearch } from "solid-icons/hi"
import { createSignal } from "solid-js"

const Search = (props) => {
  const [query, setQuery] = createSignal({ name: 'Jazz', limit: 10 })

  function submitSearch(e) {
    e.preventDefault()
    props.search(query())
  }

  return (
    <form onSubmit={submitSearch}>
      <div class="w-full h-20 bg-black border-b border-neutral-800 flex items-center justify-center mb-4">
        <input
          type="text"
          class="bg-neutral-900 rounded-lg py-1 pl-2"
          value={query().name} onChange={(e) => setQuery((prev)=>({ ...prev, name: e.target.value }))}
        />
        <button class="ml-4 hover:text-green-500">
          <HiOutlineSearch class="h-6 w-6" />
        </button>
      </div>
    </form>
  )
}

export default Search
