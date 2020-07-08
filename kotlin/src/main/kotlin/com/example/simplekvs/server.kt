package com.example.simplekvs

import org.springframework.http.MediaType
import org.springframework.web.bind.annotation.*

@RestController
class KVSController {
    var store = Store()

    @GetMapping("/")
    @ResponseBody
    fun count(): String {
        return store.begin().count().toString()
    }

    @GetMapping("/list")
    fun list(): String {
        return store.begin().keys().joinToString("\n")
    }

    @GetMapping("/k/{key}")
    @ResponseBody
    fun getOrUpdate(@PathVariable(value="key") key: String, @RequestParam(value="v") value: String?): String {
        // TODO 404 if not found
        return store.begin().update(key, value) ?: "Not Found"
    }

    @GetMapping("/d/{key}")
    @ResponseBody
    fun del(@PathVariable(value="key") key: String): String {
        return store.begin().del(key).toString()
    }

    @GetMapping("/h/{key}")
    @ResponseBody
    fun has(@PathVariable(value="key") key: String): String {
        return store.begin().has(key).toString()
    }
}