<script lang="ts">
    import {t} from "svelte-i18n";
</script>

<svelte:window on:wheel|nonpassive|preventDefault/>
<div class="loading-wrapper">
    <svg enable-background="new 0 0 64 64" height="800px"
         id="Layer_1" version="1.1" viewBox="0 0 64 64"
         width="800px" xml:space="preserve" xmlns="http://www.w3.org/2000/svg">
        <desc>Created with Sketch.</desc>
        <g id="Page-1">
	        <g id="Hammer" transform="translate(1.000000, 1.000000)">
		<path d="M35.618,30.688l-30.34,30.34
			c-0.781,0.781-2.038,0.79-2.81,0.02l-1.452-1.453c-0.771-0.771-0.762-2.028,0.019-2.81l30.34-30.339" fill="none" id="Shape_4_" stroke="#58595B" stroke-width="2"/>
                <path d="M42.601,37.847L24.216,19.462" fill="none" id="Shape_2_" stroke="#58595B"
                      stroke-width="2"
                />
                <path d="M36.943,6.734l18.385,18.385" fill="none" id="Shape_3_" stroke="#58595B"
                      stroke-width="2"/>
                <path d="M27.044,18.049l-2.828,2.827
			c-0.781,0.781-2.048,0.781-2.828,0l-2.829-2.828c-0.78-0.781-0.78-2.048,0-2.828L32.702,1.077c0.781-0.78,2.047-0.78,2.828,0
			l2.828,2.829c0.781,0.78,0.781,2.047,0.001,2.828l-2.771,2.77" fill="none" id="Shape" stroke="#58595B" stroke-width="2"/>
                <path d="M52.559,26.475l2.77-2.77
			c0.781-0.781,2.048-0.781,2.829,0l2.828,2.828c0.781,0.781,0.781,2.047,0,2.828L46.843,43.504c-0.78,0.781-2.047,0.781-2.828,0
			l-2.828-2.828c-0.781-0.781-0.781-2.048,0-2.829l2.827-2.827" fill="none" id="Shape_1_" stroke="#58595B" stroke-width="2"/>
	        </g>
        </g>
    </svg>

    <p>{$t('Loading')}...</p>
</div>

<style lang="scss">
  .loading-wrapper {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100vh;
    position: fixed;
    top: 0;
    left: 0;
    background-color: rgba(0, 0, 0, .025);
    backdrop-filter: blur(.25rem);
    justify-content: center;
    align-items: center;

    svg {
      width: 5.5rem;
      height: 5.5rem;
      animation: loading 1s infinite;

      path {
        fill: $deep-orange;
        stroke: $deep-orange;
      }
    }

    p {
      font-size: 1.5rem;
      font-weight: bold;
    }
  }

  @keyframes loading {
    from {
      transform: rotateZ(0) translateX(0);
    }
    to {
      transform: rotateZ(45deg) translateX(1rem);
    }
  }
</style>