package com.example.simplekvs.controller

import com.example.simplekvs.store.Store
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.*

// Response objects
data class CountResponse(val request_count: Long)

data class KeysResponse(val keys: List<String>)

data class BoolResponse(val key: String, val ok: Boolean)

data class KVResponse(val key: String, val value: String?)

data class NotFoundException(val key :String): Throwable()

@RestController
class KVSController {
    @ExceptionHandler(value = [(NotFoundException::class)])
    fun handleNotFound(ex: NotFoundException): ResponseEntity<KVResponse> {
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(KVResponse(ex.key, null))
    }

    @GetMapping("/")
    @ResponseBody
    fun count(): CountResponse {
        return CountResponse(Store.begin().count())
    }

    @GetMapping("/list")
    fun list(): KeysResponse {
        return KeysResponse(Store.begin().keys())
    }

    @GetMapping("/k/{key}")
    fun getOrUpdate(@PathVariable(value = "key") key: String, @RequestParam(value = "v") value: String?): KVResponse {
        val result = Store.begin().update(key, value) ?: throw NotFoundException(key)
        return KVResponse(key, result)
    }

    @GetMapping("/d/{key}")
    fun del(@PathVariable(value = "key") key: String): BoolResponse {
        return BoolResponse(key, Store.begin().del(key))
    }

    @GetMapping("/h/{key}")
    fun has(@PathVariable(value = "key") key: String): BoolResponse {
        return BoolResponse(key, Store.begin().has(key))
    }
}