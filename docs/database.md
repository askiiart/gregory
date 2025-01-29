# Database docs

Gregory's database is described as follows:

```sql
CREATE TABLE IF NOT EXISTS job_logs (
    start_time   timestamp,
    end_time    timestamp,
    duration    interval GENERATED ALWAYS AS (end_time - start_time) STORED,
    exit_code    smallint,
    job_id     text,
    revision    text,
    uuid      text,
    container_name  text GENERATED ALWAYS AS (job_id || '-' || uuid) STORED,
    log_path        text
);
```

i.e. it uses the table `job_logs`, containing the following fields:

| start_time | end_time | duration | exit_code | job_id | revision | uuid | container_name | log_path |
| ---------- | -------- | -------- | --------- | ------ | -------- | ---- | -------------- | -------- |

---

`duration` and `container_name` don't have to be inserted, as the database generates them, so they're just inserted like this:

```rs
INSERT INTO job_logs (start_time, end_time, exit_code, job_id, revision, uuid, log_path)
    VALUES ('1970-01-01 10:10:10 idkkkkk', '1970-01-01 10:11:10 idkkkkk', 1, 'packaging.librewolf.compilation', '5', 'blahblahblahblah', './data/logs/packages.librewolf.compilation/5/blahblahblahblah');
```
