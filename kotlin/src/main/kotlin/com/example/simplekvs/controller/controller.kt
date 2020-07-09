package com.example.simplekvs.controller

import com.example.simplekvs.store.Store
import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.*
import org.springframework.web.server.ResponseStatusException


@RestController
class KVSController {
    var store = Store()

    @ExceptionHandler(value = [(ResponseStatusException::class)])
    fun handleUserAlreadyExists(ex: ResponseStatusException): ResponseEntity<String> {
        return ResponseEntity.status(ex.status).body(ex.reason)
    }

    @GetMapping("/")
    @ResponseBody
    fun count(): String {
        return store.begin().count().toString()
    }

    @GetMapping("/list", produces=[MediaType.TEXT_PLAIN_VALUE])
    fun list(): String {
        return store.begin().keys().joinToString("\n")
    }

    @GetMapping("/k/{key}", produces=[MediaType.TEXT_PLAIN_VALUE])
    fun getOrUpdate(@PathVariable(value="key") key: String, @RequestParam(value="v") value: String?): String {
        return store.begin().update(key, value) ?: throw ResponseStatusException(HttpStatus.NOT_FOUND, "key not found")
    }

    @GetMapping("/d/{key}", produces=[MediaType.TEXT_PLAIN_VALUE])
    fun del(@PathVariable(value="key") key: String): String {
        return store.begin().del(key).toString()
    }

    @GetMapping("/h/{key}", produces=[MediaType.TEXT_PLAIN_VALUE])
    fun has(@PathVariable(value="key") key: String): String {
        return store.begin().has(key).toString()
    }
}