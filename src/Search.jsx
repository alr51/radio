import { useNavigate } from "@solidjs/router"
import { HiOutlineSearch, HiOutlineHeart } from "solid-icons/hi"
import { createSignal } from "solid-js"

const Search = (props) => {
  const [query, setQuery] = createSignal({ name: 'Jazz', limit: 10 })

  const navigate = useNavigate();

  function submitSearch(e) {
    e.preventDefault()
    props.search(query())
    navigate("/list")
  }

  function bookmarkList() {
    props.bookmarkList()
    navigate("/")
  }

  return (
    <form onSubmit={submitSearch} class="flex items-center justify-center">
        <input
          type="text"
          class="bg-neutral-900 rounded-lg py-1 pl-2"
          value={query().name} onChange={(e) => setQuery((prev)=>({ ...prev, name: e.target.value }))}
        />
        <button class="ml-4 hover:text-green-500">
          <HiOutlineSearch class="h-6 w-6" />
        </button>
        <button  type="button" class="ml-4 hover:text-red-500" onClick={bookmarkList}>
          <HiOutlineHeart class="h-6 w-6" />
        </button>
    </form>
  )
}

export default Search
