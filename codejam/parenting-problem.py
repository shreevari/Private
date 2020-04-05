t = int(input())


def overlap(i, j, job, array):
    if job[i][0] < array[j][1] and array[j][0] < job[i][1]:
        return True
    return False


def solve():
    jobs = []
    n = int(input())
    res = [0] * n
    jobs_dict = dict()
    for i in range(n):
        job = tuple(map(int, input().split()))
        jobs_dict[job] = i
    jobs = list(jobs_dict.keys()    )

    jobs.sort()

    c_jobs = []
    j_jobs = []
    for i in range(n):
        overlaps_c = False
        for c_index in range(len(c_jobs)):
            if overlap(i, c_index, jobs, c_jobs):
                overlaps_c = True
                break
        if not overlaps_c:
            c_jobs.append(jobs[i])
            idx = jobs_dict[jobs[i]]
            res[idx] = "C"
            continue
        overlaps_j = False
        for j_index in range(len(j_jobs)):
            if overlap(i, j_index, jobs, j_jobs):
                overlaps_j = True
                break
        if not overlaps_j:
            j_jobs.append(jobs[i])
            idx = jobs_dict[jobs[i]]
            res[idx] = "J"
            continue
        if overlaps_j and overlaps_c:
            return 'IMPOSSIBLE'
    return ''.join(res)


for __ in range(t):
    print("Case #{}: {}".format(__+1, solve()))