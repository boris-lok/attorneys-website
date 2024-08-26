<script lang="ts">
    import type {MemberDetails} from "../models/member";
    import MemberCard from "./MemberCard.svelte";
    import avatar from "../assets/avatar.svg";

    export let id: string;

    let member: MemberDetails = {
        id: id,
        name: "Lily",
        url: "",
        exp: [
            "華通國際法律事務所",
            "億光",
            "富智康"
        ],
        cert: [
            "中華民國律師高考及格"
        ],
        edu: [
            "國立中正大學法律碩士",
            "國立中正大學法律學士",
        ],
        lang: ["中文"],
        skill: ["民事及刑事訴訟"],
        description: "Lily is a talented lawyer with a strong background in law.",
        tags: [],
    }
</script>

<div class="member-container">
    <div class="avatar-content">
        <img class="avatar" src={avatar} alt="Avatar"/>
    </div>
    <div class="name-container">
        <span>
            <p>{member.name}</p>
        </span>
        <span>
            律師
        </span>
    </div>
    <div class="description">
        <p>{member.description}</p>
    </div>
    <MemberCard type="list" data={member.skill} title="Skills"/>
    <MemberCard type="list" data={member.cert} title="Certs."/>
    <MemberCard type="list" data={member.exp} title="Exp."/>
    <MemberCard type="list" data={member.edu} title="Edu."/>
    <MemberCard type="list" data={member.lang} title="Lang."/>
</div>

<style lang="scss">
  .member-container {
    display: flex;
    flex-direction: column;
    row-gap: 16px;
    padding: 32px 16px;
    width: calc(100% - 32px);
    max-width: calc(100% - 32px);
    align-items: center;

    div.avatar-content {
      img.avatar {
        width: 128px;
        height: 128px;
        border-radius: 50%;
      }
    }

    div.name-container {
      display: flex;
      flex-direction: row;
      column-gap: 4px;
      align-items: center;

      p {
        padding: 0;
        margin: 0;
      }

      span:nth-child(2) {
        font-size: 12px;
      }
    }
  }
</style>