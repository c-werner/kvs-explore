package com.example.simplekvs.store

import java.util.concurrent.atomic.AtomicLong
import java.util.concurrent.locks.ReentrantReadWriteLock
import kotlin.concurrent.withLock

class Store() {
    class ReadyStore() {
        var counter = AtomicLong(0)
        var data = HashMap<String, String>()
        var lock = ReentrantReadWriteLock()

        fun get(key: String): String? {
            lock.readLock().withLock {
                return data[key]
            }
        }

        fun set(key: String, value: String) {
            lock.writeLock().withLock {
                data[key] = value
            }
        }

        fun update(key: String, value: String?): String? {
            if (value != null) {
                set(key, value)
                return value
            }

            return get(key)
        }

        fun keys(): List<String> {
            lock.readLock().withLock {
                return data.keys.toList()
            }
        }

        fun has(key: String): Boolean {
            lock.readLock().withLock {
                return key in data
            }
        }

        fun del(key: String): Boolean {
            lock.writeLock().withLock {
                return data.remove(key) != null
            }
        }

        fun count(): Long {
            return counter.get()
        }

    }

    private var store = ReadyStore()

    fun begin(): ReadyStore {
        this.store.counter.incrementAndGet()
        return this.store
    }
}